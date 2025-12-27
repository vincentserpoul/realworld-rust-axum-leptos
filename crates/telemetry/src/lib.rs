use anyhow::Context;
use base64::Engine;
use opentelemetry::global;
use opentelemetry::metrics::{Meter, MeterProvider};
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::{MetricExporter, SpanExporter, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::resource::Resource;
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
use opentelemetry_sdk::trace::{Sampler, SdkTracerProvider, Tracer};
use std::collections::HashMap;
use std::time::Duration;
use tonic::metadata::{MetadataKey, MetadataMap, MetadataValue};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::{Layer, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;

pub mod config;

pub struct Telemetry {
    pub meter: Meter,
    _guard: TelemetryGuard,
}

pub struct TelemetryGuard {
    tracer_provider: Option<SdkTracerProvider>,
    meter_provider: Option<SdkMeterProvider>,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Some(provider) = self.tracer_provider.take() {
            let _ = provider.shutdown();
        }
        if let Some(provider) = self.meter_provider.take() {
            let _ = provider.shutdown();
        }
    }
}

pub fn init(config: &config::TelemetryConfig) -> anyhow::Result<TelemetryGuard> {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_timer(UtcTime::rfc_3339())
        .with_file(false)
        .with_line_number(false)
        .event_format(tracing_subscriber::fmt::format().compact());

    let fmt_layer = if config.log_is_json {
        fmt_layer.json().boxed()
    } else {
        fmt_layer.boxed()
    };

    let registry = tracing_subscriber::registry()
        .with(fmt_layer);

    let endpoint = config
        .otlp_endpoint
        .as_deref()
        .context("telemetry is always-on: missing telemetry.otlp_endpoint")?;

    global::set_text_map_propagator(TraceContextPropagator::new());

    let (tracer, tracer_provider) = build_otlp_tracer(config, endpoint)?;
    let meter_provider = build_otlp_meter_provider(config, endpoint)?;

    let otlp_layer = OpenTelemetryLayer::new(tracer);

    registry.with(otlp_layer).try_init()?;

    Ok(TelemetryGuard {
        tracer_provider: Some(tracer_provider),
        meter_provider: Some(meter_provider),
    })
}

pub fn init_with_meter(config: &config::TelemetryConfig) -> anyhow::Result<Telemetry> {
    let guard = init(config)?;

    let meter_provider = guard
        .meter_provider
        .as_ref()
        .context("meter provider must be initialized")?;

    let meter = meter_provider.meter("telemetry");

    Ok(Telemetry { meter, _guard: guard })
}

fn build_otlp_tracer(
    config: &config::TelemetryConfig,
    endpoint: &str,
) -> anyhow::Result<(Tracer, SdkTracerProvider)> {
    let metadata = build_otlp_metadata(config)?;

    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .with_metadata(metadata)
        .build()
        .context("building OTLP span exporter")?;

    let resource = Resource::builder()
        .with_service_name(config.service_name.clone())
        .build();

    let ratio = config.trace_sample_ratio.clamp(0.0, 1.0);
    let sampler = if config.trace_parent_based {
        Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(ratio)))
    } else {
        Sampler::TraceIdRatioBased(ratio)
    };

    let provider = SdkTracerProvider::builder()
        .with_resource(resource)
        .with_sampler(sampler)
        .with_batch_exporter(exporter)
        .build();

    let tracer = provider.tracer("telemetry");

    Ok((tracer, provider))
}

fn build_otlp_meter_provider(
    config: &config::TelemetryConfig,
    endpoint: &str,
) -> anyhow::Result<SdkMeterProvider> {
    let metadata = build_otlp_metadata(config)?;

    let exporter = MetricExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .with_metadata(metadata)
        .build()
        .context("building OTLP metric exporter")?;

    // Default is 60s; use a shorter interval so metrics show up quickly in local dev.
    let reader = PeriodicReader::builder(exporter)
        .with_interval(Duration::from_secs(5))
        .build();

    let resource = Resource::builder()
        .with_service_name(config.service_name.clone())
        .build();

    let provider = SdkMeterProvider::builder()
        .with_resource(resource)
        .with_reader(reader)
        .build();

    Ok(provider)
}


fn build_otlp_metadata(config: &config::TelemetryConfig) -> anyhow::Result<MetadataMap> {
    let headers = build_otlp_headers(config);
    let mut metadata = MetadataMap::new();

    for (key, value) in headers {
        // gRPC metadata keys are case-insensitive, but tonic expects lowercase.
        let key_lower = key.to_ascii_lowercase();

        let metadata_key = MetadataKey::from_bytes(key_lower.as_bytes())
            .with_context(|| format!("invalid gRPC metadata key: {key}"))?;
        let metadata_value = MetadataValue::try_from(value.as_str())
            .with_context(|| format!("invalid gRPC metadata value for key: {key}"))?;

        metadata.insert(metadata_key, metadata_value);
    }

    Ok(metadata)
}


fn build_otlp_headers(config: &config::TelemetryConfig) -> HashMap<String, String> {
    let mut headers = HashMap::new();

    if let (Some(username), Some(password)) = (&config.otlp_username, &config.otlp_password) {
        let token = base64::engine::general_purpose::STANDARD
            .encode(format!("{username}:{password}"));

        headers.insert("Authorization".to_string(), format!("Basic {token}"));
    }

    if let Some(org) = &config.otlp_organization {
        headers.insert("organization".to_string(), org.clone());
    }

    if let Some(stream) = &config.otlp_stream_name {
        headers.insert("stream-name".to_string(), stream.clone());
    }

    headers
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry_sdk::trace::InMemorySpanExporter;

    #[test]
    fn build_otlp_headers_includes_expected_headers() {
        let config = config::TelemetryConfig {
            service_name: "svc".to_string(),
            otlp_endpoint: Some("http://localhost:4317".to_string()),
            otlp_username: Some("user".to_string()),
            otlp_password: Some("pass".to_string()),
            otlp_organization: Some("org".to_string()),
            otlp_stream_name: Some("stream".to_string()),
            log_is_json: false,
            trace_sample_ratio: 1.0,
            trace_parent_based: true,
        };

        let headers = build_otlp_headers(&config);

        assert_eq!(
            headers.get("organization").map(String::as_str),
            Some("org")
        );
        assert_eq!(
            headers.get("stream-name").map(String::as_str),
            Some("stream")
        );

        // "user:pass" base64 is "dXNlcjpwYXNz"
        assert_eq!(
            headers.get("Authorization").map(String::as_str),
            Some("Basic dXNlcjpwYXNz")
        );
    }

    #[test]
    fn build_otlp_headers_omits_auth_if_partial_credentials() {
        let config = config::TelemetryConfig {
            service_name: "svc".to_string(),
            otlp_endpoint: Some("http://localhost:4317".to_string()),
            otlp_username: Some("user".to_string()),
            otlp_password: None,
            otlp_organization: None,
            otlp_stream_name: None,
            log_is_json: false,
            trace_sample_ratio: 1.0,
            trace_parent_based: true,
        };

        let headers = build_otlp_headers(&config);
        assert!(!headers.contains_key("Authorization"));
    }

    #[test]
    fn build_otlp_metadata_lowercases_keys() {
        let config = config::TelemetryConfig {
            service_name: "svc".to_string(),
            otlp_endpoint: Some("http://localhost:4317".to_string()),
            otlp_username: None,
            otlp_password: None,
            otlp_organization: Some("org".to_string()),
            otlp_stream_name: Some("stream".to_string()),
            log_is_json: false,
            trace_sample_ratio: 1.0,
            trace_parent_based: true,
        };

        let metadata = build_otlp_metadata(&config).expect("metadata should build");

        // tonic metadata keys must be lowercase.
        assert_eq!(
            metadata.get("organization").and_then(|v| v.to_str().ok()),
            Some("org")
        );
        assert_eq!(
            metadata.get("stream-name").and_then(|v| v.to_str().ok()),
            Some("stream")
        );
    }

    #[test]
    fn e2e_tracing_span_is_exported_to_inmemory_exporter() {
        let exporter = InMemorySpanExporter::default();
        let provider = SdkTracerProvider::builder()
            .with_resource(Resource::builder().with_service_name("telemetry-test").build())
            .with_simple_exporter(exporter.clone())
            .build();

        let tracer = provider.tracer("telemetry-test");
        let otel_layer = OpenTelemetryLayer::new(tracer);

        let subscriber = tracing_subscriber::registry().with(otel_layer);
        let dispatch = tracing::Dispatch::new(subscriber);

        tracing::dispatcher::with_default(&dispatch, || {
            let span = tracing::info_span!("e2e_span");
            let _guard = span.enter();
            tracing::info!("hello");
        });

        provider.force_flush().expect("force_flush should succeed");

        let spans = exporter
            .get_finished_spans()
            .expect("in-memory exporter should return spans");

        assert!(
            !spans.is_empty(),
            "expected at least one exported span"
        );
    }
}

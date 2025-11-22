use anyhow::Context;
use common_config::TelemetryConfig;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::resource::Resource;
use opentelemetry_sdk::trace::{SdkTracerProvider, Tracer};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::{Layer, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;

pub fn init(config: &TelemetryConfig) -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_timer(UtcTime::rfc_3339())
        .with_file(false)
        .with_line_number(false)
        .event_format(tracing_subscriber::fmt::format().compact());

    let fmt_layer = if config.log_json {
        fmt_layer.json().boxed()
    } else {
        fmt_layer.boxed()
    };

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    if let Some(endpoint) = &config.otlp_endpoint {
        global::set_text_map_propagator(TraceContextPropagator::new());
        let tracer = build_otlp_tracer(config, endpoint)?;
        let otlp_layer = OpenTelemetryLayer::new(tracer);
        registry.with(otlp_layer).try_init()?;
    } else {
        registry.try_init()?;
    }

    Ok(())
}

fn build_otlp_tracer(config: &TelemetryConfig, endpoint: &str) -> anyhow::Result<Tracer> {
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()
        .context("building OTLP span exporter")?;

    let resource = Resource::builder()
        .with_service_name(config.service_name.clone())
        .build();

    let provider = SdkTracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .build();

    let tracer = provider.tracer(config.service_name.clone());
    global::set_tracer_provider(provider);

    Ok(tracer)
}

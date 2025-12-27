
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TelemetryConfig {
    #[serde(default)]
    pub service_name: String,
    pub otlp_endpoint: Option<String>,
    #[serde(default)]
    pub otlp_username: Option<String>,
    #[serde(default)]
    pub otlp_password: Option<String>,
    #[serde(default)]
    pub otlp_organization: Option<String>,
    #[serde(default)]
    pub otlp_stream_name: Option<String>,
    #[serde(default)]
    pub log_is_json: bool,

    #[serde(default = "TelemetryConfig::default_trace_sample_ratio")]
    pub trace_sample_ratio: f64,
    #[serde(default = "TelemetryConfig::default_trace_parent_based")]
    pub trace_parent_based: bool,
}

impl TelemetryConfig {
    fn default_trace_sample_ratio() -> f64 {
        1.0
    }

    fn default_trace_parent_based() -> bool {
        true
    }
}
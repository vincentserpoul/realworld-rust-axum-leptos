use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use serde_with::{DurationSeconds, serde_as};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConfigLoadError>;

pub const ENV_CONFIG_PATH: &str = "APP_CONFIG_PATH";
pub const DEFAULT_CONFIG_FILE: &str = "config/base";

#[derive(Debug, Error)]
pub enum ConfigLoadError {
    #[error(transparent)]
    Build(#[from] ConfigError),
    #[error("invalid socket address: {0}")]
    Addr(#[from] std::net::AddrParseError),
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub opa: OpaConfig,
    pub iggy: IggyConfig,
    pub restate: RestateConfig,
    pub telemetry: TelemetryConfig,
    pub security: SecurityConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let mut builder = Config::builder();

        if let Ok(path) = env::var(ENV_CONFIG_PATH) {
            builder = builder.add_source(File::from(PathBuf::from(path)));
        }

        builder = builder.add_source(File::with_name(DEFAULT_CONFIG_FILE).required(false));
        builder = builder.add_source(Environment::with_prefix("APP").separator("__"));

        Ok(builder.build()?.try_deserialize()?)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "ServerConfig::default_host")]
    pub host: String,
    #[serde(default = "ServerConfig::default_port")]
    pub port: u16,
    #[serde(default = "ServerConfig::default_sse_buffer")]
    pub sse_buffer: usize,
    #[serde(default)]
    pub tls: Option<TlsConfig>,
}

impl ServerConfig {
    fn default_host() -> String {
        "127.0.0.1".into()
    }

    const fn default_port() -> u16 {
        8080
    }

    const fn default_sse_buffer() -> usize {
        1024
    }

    pub fn addr(&self) -> Result<SocketAddr> {
        Ok(format!("{}:{}", self.host, self.port).parse()?)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TlsConfig {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default = "DatabaseConfig::default_pool_size")]
    pub pool_size: usize,
}

impl DatabaseConfig {
    const fn default_pool_size() -> usize {
        10
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpaConfig {
    pub base_url: String,
    pub policy_path: String,
    #[serde(default = "OpaConfig::default_cache_ttl_seconds")]
    pub cache_ttl_seconds: u64,
}

impl OpaConfig {
    const fn default_cache_ttl_seconds() -> u64 {
        30
    }

    pub fn cache_ttl(&self) -> Duration {
        Duration::from_secs(self.cache_ttl_seconds)
    }

    pub fn policy_url(&self) -> String {
        format!(
            "{}/v1/data{}",
            self.base_url.trim_end_matches('/'),
            self.policy_path
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct IggyConfig {
    pub connection_string: String,
    #[serde(default = "IggyConfig::default_stream")]
    pub stream: String,
    #[serde(default = "IggyConfig::default_topic")]
    pub topic: String,
    #[serde(default = "IggyConfig::default_batch_size")]
    pub batch_size: u32,
    #[serde(default = "IggyConfig::default_linger_ms")]
    pub linger_ms: u64,
}

impl IggyConfig {
    fn default_stream() -> String {
        "app_stream".into()
    }

    fn default_topic() -> String {
        "outbox".into()
    }

    const fn default_batch_size() -> u32 {
        64
    }

    const fn default_linger_ms() -> u64 {
        5
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RestateConfig {
    pub workflow_bind_address: String,
    pub ingress_url: String,
    #[serde(default = "RestateConfig::default_namespace")]
    pub namespace: String,
}

impl RestateConfig {
    fn default_namespace() -> String {
        "default".into()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TelemetryConfig {
    #[serde(default = "TelemetryConfig::default_service_name")]
    pub service_name: String,
    pub otlp_endpoint: Option<String>,
    #[serde(default)]
    pub log_json: bool,
}

impl TelemetryConfig {
    fn default_service_name() -> String {
        "realworld-rust-axum-leptos".into()
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct SecurityConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub required_key_id: Option<String>,
    #[serde(default)]
    pub ed25519_public_keys: Vec<String>,
    #[serde(default = "SecurityConfig::default_skew")]
    #[serde_as(as = "DurationSeconds<u64>")]
    pub clock_skew_tolerance: Duration,
}

impl SecurityConfig {
    fn default_skew() -> Duration {
        Duration::from_secs(5)
    }
}

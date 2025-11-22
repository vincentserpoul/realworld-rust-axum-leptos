use anyhow::Context;
use common_config::OpaConfig;
use moka::future::Cache;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tracing::{instrument, warn};

#[derive(Clone)]
pub struct OpaAuthorizer {
    client: Client,
    config: OpaConfig,
    cache: Cache<String, AuthzDecision>,
}

impl OpaAuthorizer {
    pub fn new(config: OpaConfig) -> Self {
        let cache = Cache::builder()
            .time_to_live(config.cache_ttl())
            .max_capacity(10_000)
            .build();

        Self {
            client: Client::new(),
            config,
            cache,
        }
    }

    #[instrument(skip(self), fields(subject = %request.subject, action = %request.action))]
    pub async fn authorize(&self, request: AuthzRequest) -> Result<AuthzDecision, AuthzError> {
        let cache_key = request.cache_key();
        let decision = self
            .cache
            .try_get_with(cache_key.clone(), async move {
                Self::evaluate(self.client.clone(), &self.config, request).await
            })
            .await
            .map_err(|e| AuthzError::Cache(anyhow::Error::new(e)))?;

        if decision.allow {
            Ok(decision)
        } else {
            Err(AuthzError::Denied(decision))
        }
    }

    async fn evaluate(
        client: Client,
        config: &OpaConfig,
        request: AuthzRequest,
    ) -> Result<AuthzDecision, AuthzError> {
        let payload = OpaInput { input: request };
        let response = client
            .post(config.policy_url())
            .json(&payload)
            .send()
            .await
            .context("OPA request failed")?;

        if !response.status().is_success() {
            warn!(status = %response.status(), "OPA returned non-success status");
            return Ok(AuthzDecision::deny("OPA HTTP failure"));
        }

        let body: OpaResponse = response
            .json()
            .await
            .context("OPA response parsing failed")?;

        Ok(body.into())
    }
}

#[derive(Debug, Clone, Serialize)]
struct OpaInput {
    input: AuthzRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthzRequest {
    pub subject: String,
    pub action: String,
    pub resource: String,
    #[serde(default)]
    pub context: Value,
}

impl AuthzRequest {
    fn cache_key(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.subject, self.action, self.resource, self.context,
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
struct OpaResponse {
    result: OpaResult,
}

#[derive(Debug, Clone, Deserialize)]
struct OpaResult {
    allow: bool,
    #[serde(default)]
    reason: Option<String>,
}

impl From<OpaResponse> for AuthzDecision {
    fn from(value: OpaResponse) -> Self {
        if value.result.allow {
            AuthzDecision::allow(value.result.reason)
        } else {
            AuthzDecision::deny(value.result.reason.unwrap_or_else(|| "OPA denied".into()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthzDecision {
    pub allow: bool,
    pub reason: Option<String>,
}

impl AuthzDecision {
    pub fn allow(reason: Option<String>) -> Self {
        Self {
            allow: true,
            reason,
        }
    }

    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            allow: false,
            reason: Some(reason.into()),
        }
    }
}

#[derive(Debug, Error)]
pub enum AuthzError {
    #[error("OPA denied request")]
    Denied(AuthzDecision),
    #[error(transparent)]
    Cache(anyhow::Error),
    #[error(transparent)]
    Transport(#[from] anyhow::Error),
}

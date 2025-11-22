use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, anyhow};
use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use chrono::{DateTime, Duration as ChronoDuration, TimeZone, Utc};
use common_config::SecurityConfig;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use http_problem::ProblemDetails;

const HEADER_SIGNATURE: &str = "x-signature";
const HEADER_TIMESTAMP: &str = "x-signed-timestamp";
const HEADER_KEY_ID: &str = "x-key-id";

#[derive(Clone)]
pub struct SignedRequestVerifier {
    keys: Arc<HashMap<String, VerifyingKey>>,
    tolerance: Duration,
    required_key_id: Option<String>,
}

impl SignedRequestVerifier {
    pub fn maybe_from_config(config: &SecurityConfig) -> anyhow::Result<Option<Self>> {
        if !config.enabled || config.ed25519_public_keys.is_empty() {
            return Ok(None);
        }

        let mut keys = HashMap::new();
        for entry in &config.ed25519_public_keys {
            let (key_id, encoded) = entry
                .split_once(':')
                .ok_or_else(|| anyhow!("invalid key entry, use <id>:<base64> format"))?;
            let raw = BASE64
                .decode(encoded.as_bytes())
                .context("invalid base64 in ed25519 key")?;
            let bytes: [u8; 32] = raw
                .try_into()
                .map_err(|_| anyhow!("ed25519 public keys must be 32 bytes"))?;
            let key = VerifyingKey::from_bytes(&bytes)
                .map_err(|_| anyhow!("invalid ed25519 public key"))?;
            keys.insert(key_id.to_string(), key);
        }

        Ok(Some(Self {
            keys: Arc::new(keys),
            tolerance: config.clock_skew_tolerance,
            required_key_id: config.required_key_id.clone(),
        }))
    }

    async fn verify(&self, request: Request<Body>) -> Result<Request<Body>, ProblemDetails> {
        let key_id = self.extract_header(&request, HEADER_KEY_ID)?;
        if let Some(required) = &self.required_key_id {
            if &key_id != required {
                return Err(unauthorized("unexpected key id"));
            }
        }

        let verifier = self
            .keys
            .get(&key_id)
            .ok_or_else(|| unauthorized("unknown key id"))?;
        let signature = self.decode_signature(&request)?;
        let timestamp_raw = self.extract_header(&request, HEADER_TIMESTAMP)?;
        let timestamp = parse_timestamp(&timestamp_raw)?;
        ensure_tolerance(timestamp, self.tolerance)?;

        let (parts, body) = request.into_parts();
        let body_bytes = buffer_body(body).await?;
        let canonical = canonical_message(&parts, &timestamp_raw, &body_bytes);
        verifier
            .verify(&canonical, &signature)
            .map_err(|_| unauthorized("signature mismatch"))?;

        let rebuilt = Request::from_parts(parts, Body::from(body_bytes));
        Ok(rebuilt)
    }

    fn extract_header(
        &self,
        request: &Request<Body>,
        name: &str,
    ) -> Result<String, ProblemDetails> {
        request
            .headers()
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(|s| s.to_string())
            .ok_or_else(|| unauthorized(&format!("missing header {name}")))
    }

    fn decode_signature(&self, request: &Request<Body>) -> Result<Signature, ProblemDetails> {
        let encoded = self.extract_header(request, HEADER_SIGNATURE)?;
        let raw = BASE64
            .decode(encoded.as_bytes())
            .map_err(|_| unauthorized("invalid signature encoding"))?;
        let bytes: [u8; 64] = raw
            .try_into()
            .map_err(|_| unauthorized("invalid signature length"))?;
        Ok(Signature::from_bytes(&bytes))
    }
}

pub async fn signed_request_middleware(
    State(verifier): State<Option<SignedRequestVerifier>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, ProblemDetails> {
    if let Some(verifier) = &verifier {
        let request = verifier.verify(request).await?;
        Ok(next.run(request).await)
    } else {
        Ok(next.run(request).await)
    }
}

fn canonical_message(parts: &http::request::Parts, timestamp: &str, body: &[u8]) -> Vec<u8> {
    let method = parts.method.as_str();
    let path = parts
        .uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or_else(|| parts.uri.path());
    let mut canonical = format!("{timestamp}\n{method}\n{path}\n").into_bytes();
    canonical.extend_from_slice(body);
    canonical
}

fn parse_timestamp(value: &str) -> Result<DateTime<Utc>, ProblemDetails> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
        return Ok(dt.with_timezone(&Utc));
    }

    if let Ok(secs) = value.parse::<i64>() {
        if let Some(dt) = Utc.timestamp_opt(secs, 0).single() {
            return Ok(dt);
        }
    }

    Err(unauthorized("invalid timestamp"))
}

fn ensure_tolerance(timestamp: DateTime<Utc>, tolerance: Duration) -> Result<(), ProblemDetails> {
    let chrono_tol =
        ChronoDuration::from_std(tolerance).unwrap_or_else(|_| ChronoDuration::seconds(5));
    let delta = (Utc::now() - timestamp).abs();
    if delta > chrono_tol {
        Err(unauthorized("timestamp outside allowed skew"))
    } else {
        Ok(())
    }
}

fn unauthorized(detail: &str) -> ProblemDetails {
    ProblemDetails::new(StatusCode::UNAUTHORIZED).with_detail(detail.to_string())
}

async fn buffer_body(body: Body) -> Result<Bytes, ProblemDetails> {
    axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| unauthorized("unable to read body for verification"))
}

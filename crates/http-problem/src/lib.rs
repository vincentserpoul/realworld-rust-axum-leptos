use axum::Json;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::error::Error as StdError;
use std::fmt;
use thiserror::Error;
use utoipa::ToSchema;

pub type ProblemResult<T> = std::result::Result<T, ProblemDetails>;

#[derive(Debug, Error)]
pub enum ProblemError {
    #[error("problem: {0}")]
    Problem(#[from] ProblemDetails),
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProblemDetails {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub title: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(flatten, skip_serializing_if = "map_is_empty")]
    pub extensions: Map<String, Value>,
}

impl ProblemDetails {
    pub fn new(status: StatusCode) -> Self {
        Self {
            kind: Some(format!("about:blank#{}", status.as_str())),
            title: status
                .canonical_reason()
                .unwrap_or("HTTP Problem")
                .to_string(),
            status: status.as_u16(),
            detail: None,
            instance: None,
            extensions: Map::new(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }

    pub fn with_extension(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }

    pub fn from_anyhow(status: StatusCode, error: &anyhow::Error) -> Self {
        Self::new(status).with_detail(error.to_string())
    }
}

fn map_is_empty(map: &Map<String, Value>) -> bool {
    map.is_empty()
}

impl fmt::Display for ProblemDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (status {}", self.title, self.status)?;
        if let Some(detail) = &self.detail {
            write!(f, ": {detail}")?;
        }
        if let Some(kind) = &self.kind {
            write!(f, ", type {kind}")?;
        }
        write!(f, ")")
    }
}

impl StdError for ProblemDetails {}

impl IntoResponse for ProblemDetails {
    fn into_response(self) -> Response {
        let mut response = (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self.clone()),
        )
            .into_response();
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/problem+json"),
        );
        response
    }
}

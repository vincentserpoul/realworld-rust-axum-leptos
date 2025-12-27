pub mod api;

use crate::state::AppState;
use axum::{extract::{MatchedPath, State}, middleware, routing::get, Router};
use opentelemetry::{metrics::{Counter, Histogram, Meter}, KeyValue};
use std::time::Instant;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::Level;

pub fn router<U, A, C>(state: AppState<U, A, C>, meter: Meter) -> Router
where
    U: domain::repositories::UsersRepository + Clone + 'static,
    A: domain::repositories::ArticlesRepository + Clone + 'static,
    C: domain::repositories::CommentsRepository + Clone + 'static,
{
    let http_metrics = HttpMetrics::new(meter);

    Router::<AppState<U, A, C>>::new()
        .route("/health", get(health))
        .nest("/api", api::router())
        // Route-layer runs after matching, so MatchedPath is available.
        .route_layer(middleware::from_fn_with_state(http_metrics, record_http_metrics))
        .layer(
            TraceLayer::new_for_http()
                // Default is DEBUG; use INFO so spans exist with RUST_LOG=info.
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO)),
        )
        .with_state::<()>(state)
}

#[derive(Clone)]
struct HttpMetrics {
    request_count: Counter<u64>,
    request_duration_s: Histogram<f64>,
}

impl HttpMetrics {
    fn new(meter: Meter) -> Self {
        let request_count = meter
            .u64_counter("http.server.request.count")
            .with_description("Total number of HTTP requests")
            .build();

        // OpenTelemetry semantic conventions use seconds for durations.
        let request_duration_s = meter
            .f64_histogram("http.server.request.duration")
            .with_description("HTTP request duration")
            .with_unit("s")
            .build();

        Self {
            request_count,
            request_duration_s,
        }
    }
}

async fn record_http_metrics(
    State(metrics): State<HttpMetrics>,
    req: axum::http::Request<axum::body::Body>,
    next: middleware::Next,
) -> axum::response::Response {
    let start = Instant::now();
    let method = req.method().to_string();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str().to_string())
        .unwrap_or_else(|| req.uri().path().to_string());

    let response = next.run(req).await;

    let status = response.status().as_u16() as i64;
    let duration_s = start.elapsed().as_secs_f64();

    let attrs = [
        KeyValue::new("http.request.method", method),
        KeyValue::new("http.route", route),
        KeyValue::new("http.response.status_code", status),
    ];

    metrics.request_count.add(1, &attrs);
    metrics.request_duration_s.record(duration_s, &attrs);

    response
}

async fn health() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let state = AppState::default();
        let app = Router::new()
            .route("/health", get(health))
            .with_state(state);
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
}

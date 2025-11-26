pub mod api;

use crate::state::AppState;
use axum::{routing::get, Router};

pub fn router<U, A, C>(state: AppState<U, A, C>) -> Router
where
    U: domain::repositories::UsersRepository + Clone + 'static,
    A: domain::repositories::ArticlesRepository + Clone + 'static,
    C: domain::repositories::CommentsRepository + Clone + 'static,
{
    Router::<AppState<U, A, C>>::new()
        .route("/health", get(health))
        .nest("/api", api::router())
        .with_state::<()>(state)
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

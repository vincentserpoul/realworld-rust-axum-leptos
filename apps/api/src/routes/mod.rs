pub mod api;

use crate::state::AppState;
use axum::{routing::get, Router};

pub fn router(state: AppState) -> Router {
    Router::<AppState>::new()
        .route("/health", get(health))
        .nest("/api", api::router())
        .with_state::<()>(state)
}

async fn health() -> &'static str {
    "ok"
}

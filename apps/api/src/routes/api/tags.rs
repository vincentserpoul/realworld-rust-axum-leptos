use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use serde::Serialize;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_tags))
}

#[derive(Debug, Serialize)]
struct TagsResponse {
    tags: Vec<String>,
}

async fn get_tags(State(state): State<AppState>) -> impl IntoResponse {
    let tags = state
        .tags
        .read()
        .await
        .as_slice()
        .iter()
        .map(|tag| tag.as_str().to_owned())
        .collect();

    Json(TagsResponse { tags })
}

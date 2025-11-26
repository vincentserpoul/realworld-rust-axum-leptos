use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use serde::Serialize;

use crate::state::AppState;

pub fn router<U, A, C>() -> Router<AppState<U, A, C>>
where
    U: domain::repositories::UsersRepository + Clone + 'static,
    A: domain::repositories::ArticlesRepository + Clone + 'static,
    C: domain::repositories::CommentsRepository + Clone + 'static,
{
    Router::<AppState<U, A, C>>::new().route("/", get(get_tags))
}

#[derive(Debug, Serialize)]
struct TagsResponse {
    tags: Vec<String>,
}

async fn get_tags<U, A, C>(State(state): State<AppState<U, A, C>>) -> impl IntoResponse
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_tags() {
        let state = AppState::default();
        let app = router().with_state(state);
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
}

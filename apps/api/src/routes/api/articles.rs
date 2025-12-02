use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post},
};
use chrono::Utc;
use domain::{
    ArticleEnvelope, ArticlesEnvelope, CommentEnvelope, CommentId, CommentsEnvelope,
    use_cases::{
        create_article, create_comment, delete_article, delete_comment, favorite_article,
        feed_articles, get_article, list_articles, list_comments, unfavorite_article,
        update_article, CreateArticleInput, CreateCommentInput, FeedArticlesInput,
        ListArticlesInput, UpdateArticleInput,
    },
};
use serde::Deserialize;

use crate::{
    auth::CurrentUser,
    error::{ApiError, ApiResult},
    state::AppState,
};

pub fn router<U, A, C>() -> Router<AppState<U, A, C>>
where
    U: domain::repositories::UsersRepository + Clone + 'static,
    A: domain::repositories::ArticlesRepository + Clone + 'static,
    C: domain::repositories::CommentsRepository + Clone + 'static,
{
    Router::<AppState<U, A, C>>::new()
        .route("/", get(list_articles_handler).post(create_article_handler))
        .route("/feed", get(feed_articles_handler))
        .route(
            "/{slug}",
            get(get_article_handler).put(update_article_handler).delete(delete_article_handler),
        )
        .route(
            "/{slug}/favorite",
            post(favorite_article_handler).delete(unfavorite_article_handler),
        )
        .route("/{slug}/comments", get(list_comments_handler).post(create_comment_handler))
        .route("/{slug}/comments/{id}", delete(delete_comment_handler))
}

#[derive(Debug, Default, Deserialize)]
struct ListQuery {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

#[derive(Debug, Default, Deserialize)]
struct FeedQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}

async fn list_articles_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Query(query): Query<ListQuery>,
    _current_user: Option<CurrentUser>,
) -> ApiResult<Json<ArticlesEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let input = ListArticlesInput {
        tag: query.tag,
        author: query.author,
        favorited: query.favorited,
        limit: query.limit,
        offset: query.offset,
    };

    let envelope = list_articles(&state.use_cases.articles_repo, input).await?;
    Ok(Json(envelope))
}

async fn feed_articles_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Query(query): Query<FeedQuery>,
    CurrentUser { user, .. }: CurrentUser,
) -> ApiResult<Json<ArticlesEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let input = FeedArticlesInput {
        limit: query.limit,
        offset: query.offset,
    };

    let envelope = feed_articles(&state.use_cases.articles_repo, user.id, input).await?;
    Ok(Json(envelope))
}

#[derive(Debug, Deserialize)]
struct CreateArticleRequest {
    article: ArticlePayload,
}

#[derive(Debug, Deserialize)]
struct ArticlePayload {
    title: String,
    description: String,
    body: String,
    #[serde(default, rename = "tagList")]
    tag_list: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateArticleRequest {
    article: UpdateArticlePayload,
}

#[derive(Debug, Deserialize)]
struct UpdateArticlePayload {
    title: Option<String>,
    description: Option<String>,
    body: Option<String>,
    #[serde(rename = "tagList")]
    tag_list: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct CreateCommentRequest {
    comment: CommentPayload,
}

#[derive(Debug, Deserialize)]
struct CommentPayload {
    body: String,
}

async fn create_article_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Json(req): Json<CreateArticleRequest>,
) -> ApiResult<Json<ArticleEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let input = CreateArticleInput {
        title: req.article.title,
        description: req.article.description,
        body: req.article.body,
        tag_list: req.article.tag_list,
    };

    let author_profile = user.to_profile(false);
    let view = create_article(
        &state.use_cases.articles_repo,
        user.id,
        author_profile,
        input,
        Utc::now(),
    )
    .await?;

    Ok(Json(ArticleEnvelope::from(view)))
}

async fn get_article_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Path(slug): Path<String>,
    current_user: Option<CurrentUser>,
) -> ApiResult<Json<ArticleEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let viewer_id = current_user.as_ref().map(|current| current.user.id);

    let view = get_article(
        &state.use_cases.users_repo,
        &state.use_cases.articles_repo,
        &slug,
        viewer_id,
    )
    .await
    .map_err(|_| ApiError::not_found("article"))?;

    Ok(Json(ArticleEnvelope::from(view)))
}

async fn update_article_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
    Json(req): Json<UpdateArticleRequest>,
) -> ApiResult<Json<ArticleEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let input = UpdateArticleInput {
        title: req.article.title,
        description: req.article.description,
        body: req.article.body,
        tag_list: req.article.tag_list,
    };

    let view = update_article(
        &state.use_cases.users_repo,
        &state.use_cases.articles_repo,
        &slug,
        user.id,
        input,
        Utc::now(),
    )
    .await
    .map_err(|e| match e {
        domain::DomainError::NotFound { .. } => ApiError::not_found("article"),
        domain::DomainError::UnauthorizedAction => {
            ApiError::unauthorized("cannot edit another user's article")
        }
        _ => ApiError::from(e),
    })?;

    Ok(Json(ArticleEnvelope::from(view)))
}

async fn delete_article_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
) -> ApiResult<()>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    delete_article(&state.use_cases.articles_repo, &slug, user.id)
        .await
        .map_err(|_| ApiError::not_found("article"))?;

    Ok(())
}

async fn favorite_article_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
) -> ApiResult<Json<ArticleEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let view = favorite_article(
        &state.use_cases.users_repo,
        &state.use_cases.articles_repo,
        &slug,
        user.id,
    )
    .await
    .map_err(|_| ApiError::not_found("article"))?;

    Ok(Json(ArticleEnvelope::from(view)))
}

async fn unfavorite_article_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
) -> ApiResult<Json<ArticleEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let view = unfavorite_article(
        &state.use_cases.users_repo,
        &state.use_cases.articles_repo,
        &slug,
        user.id,
    )
    .await
    .map_err(|_| ApiError::not_found("article"))?;

    Ok(Json(ArticleEnvelope::from(view)))
}

async fn list_comments_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Path(slug): Path<String>,
    current_user: Option<CurrentUser>,
) -> ApiResult<Json<CommentsEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let viewer_id = current_user.as_ref().map(|current| current.user.id);

    let views = list_comments(
        &state.use_cases.users_repo,
        &state.use_cases.articles_repo,
        &state.use_cases.comments_repo,
        &slug,
        viewer_id,
    )
    .await
    .map_err(|_| ApiError::not_found("article"))?;

    Ok(Json(CommentsEnvelope::from(views)))
}

async fn create_comment_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
    Json(req): Json<CreateCommentRequest>,
) -> ApiResult<Json<CommentEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let input = CreateCommentInput {
        body: req.comment.body,
    };

    let view = create_comment(
        &state.use_cases.users_repo,
        &state.use_cases.articles_repo,
        &state.use_cases.comments_repo,
        &slug,
        user.id,
        input,
        Utc::now(),
    )
    .await
    .map_err(|_| ApiError::not_found("article"))?;

    Ok(Json(CommentEnvelope::from(view)))
}

async fn delete_comment_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Path((slug, id)): Path<(String, i64)>,
) -> ApiResult<()>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let comment_id = CommentId::new(id);

    delete_comment(
        &state.use_cases.articles_repo,
        &state.use_cases.comments_repo,
        &slug,
        comment_id,
        user.id,
    )
    .await
    .map_err(|_| ApiError::not_found("comment"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use domain::{Email, PasswordHash, User, Username, UserId};

    fn create_test_user(id: UserId, username: &str, email: &str) -> User {
        User::new(
            id,
            Email::parse(email).unwrap(),
            Username::new(username).unwrap(),
            PasswordHash::new("hash".to_string()).unwrap(),
            chrono::Utc::now(),
        )
    }

    #[test]
    fn test_create_test_user() {
        let user = create_test_user(UserId::random(), "test", "test@example.com");
        assert_eq!(user.username.as_str(), "test");
    }
}

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post},
};
use chrono::Utc;
use domain::{
    Article, ArticleChanges, ArticleDraft, ArticleEnvelope, ArticleFilters, ArticleId,
    ArticlesEnvelope, Comment, CommentDraft, CommentEnvelope, CommentId, CommentsEnvelope,
    FeedFilters, Pagination, TagList,
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
        .route("/", get(list_articles).post(create_article))
        .route("/feed", get(feed_articles))
        .route(
            "/:slug",
            get(get_article).put(update_article).delete(delete_article),
        )
        .route(
            "/:slug/favorite",
            post(favorite_article).delete(unfavorite_article),
        )
        .route("/:slug/comments", get(list_comments).post(create_comment))
        .route("/:slug/comments/:id", delete(delete_comment))
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

async fn list_articles<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Query(query): Query<ListQuery>,
    _current_user: Option<CurrentUser>,
) -> ApiResult<Json<ArticlesEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let pagination = Pagination::new(query.limit, query.offset)?;
    let filters = ArticleFilters::new(query.tag, query.author, query.favorited, Some(pagination))?;

    let envelope = state.use_cases.articles_repo.list_articles(filters).await?;
    Ok(Json(envelope))
}

async fn feed_articles<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Query(query): Query<FeedQuery>,
    CurrentUser { user, .. }: CurrentUser,
) -> ApiResult<Json<ArticlesEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let pagination = Pagination::new(query.limit, query.offset)?;
    let filters = FeedFilters::new(Some(pagination));
    let envelope = state.use_cases.articles_repo.feed_articles(user.id, filters).await?;
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

async fn create_article<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Json(req): Json<CreateArticleRequest>,
) -> ApiResult<Json<ArticleEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let ArticlePayload {
        title,
        description,
        body,
        tag_list,
    } = req.article;

    let tags = TagList::new(tag_list)?;
    let draft = ArticleDraft::new(title, description, body, tags)?;
    
    let profile = user.to_profile(false);
    let (article, _view) = Article::create_from_draft(
        ArticleId::random(),
        user.id,
        draft,
        profile,
        Utc::now(),
    )?;
    
    let created = state.use_cases.articles_repo.create_article(article).await?;
    let profile = user.to_profile(false);
    let view = Article::build_view(&created, profile, false);
    Ok(Json(ArticleEnvelope::from(view)))
}

async fn get_article<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Path(slug): Path<String>,
    current_user: Option<CurrentUser>,
) -> ApiResult<Json<ArticleEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let article = state.use_cases.articles_repo.get_article_by_slug(&slug).await?
        .ok_or_else(|| ApiError::not_found("article"))?;

    let viewer_id = current_user.as_ref().map(|current| current.user.id);
    let author = state.use_cases.users_repo.get_user_by_id(article.author_id).await?
        .ok_or_else(|| ApiError::not_found("author"))?;
    let following = if let Some(viewer) = viewer_id {
        state.use_cases.users_repo.is_following(viewer, article.author_id).await.unwrap_or(false)
    } else {
        false
    };
    let profile = author.to_profile(following);
    let favorited = if let Some(viewer) = viewer_id {
        state.use_cases.articles_repo.is_favorited(viewer, article.id).await.unwrap_or(false)
    } else {
        false
    };
    let view = Article::build_view(&article, profile, favorited);
    Ok(Json(ArticleEnvelope::from(view)))
}

async fn update_article<U, A, C>(
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
    let UpdateArticlePayload {
        title,
        description,
        body,
        tag_list,
    } = req.article;

    let mut article = state.use_cases.articles_repo
        .get_article_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::not_found("article"))?;

    if article.author_id != user.id {
        return Err(ApiError::unauthorized("cannot edit another user's article"));
    }

    let mut changes = ArticleChanges {
        title,
        description,
        body,
        ..Default::default()
    };
    if let Some(tags) = tag_list {
        changes.tag_list = Some(TagList::new(tags)?);
    }

    article.apply_changes(changes, Utc::now())?;
    let updated = state.use_cases.articles_repo.update_article(article).await?;

    let favorited = state.use_cases.articles_repo
        .is_favorited(user.id, updated.id)
        .await
        .unwrap_or(false);
    let profile = user.to_profile(false);
    let view = updated.to_view(profile, favorited);
    Ok(Json(ArticleEnvelope::from(view)))
}

async fn delete_article<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
) -> ApiResult<()>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let article = state.use_cases.articles_repo
        .get_article_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::not_found("article"))?;

    if article.author_id != user.id {
        return Err(ApiError::not_found("article"));
    }

    state.use_cases.articles_repo.delete_article(article.id).await?;
    Ok(())
}

async fn favorite_article<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
) -> ApiResult<Json<ArticleEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let article = state.use_cases.articles_repo
        .get_article_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::not_found("article"))?;

    state.use_cases.articles_repo.favorite_article(user.id, article.id).await?;

    let updated = state.use_cases.articles_repo
        .get_article_by_id(article.id)
        .await?
        .unwrap();

    let author = state.use_cases.users_repo
        .get_user_by_id(updated.author_id)
        .await?
        .ok_or_else(|| ApiError::not_found("author"))?;
    let following = state.use_cases.users_repo
        .is_following(user.id, author.id)
        .await
        .unwrap_or(false);
    let profile = author.to_profile(following);
    let view = updated.to_view(profile, true);
    Ok(Json(ArticleEnvelope::from(view)))
}

async fn unfavorite_article<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
) -> ApiResult<Json<ArticleEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let article = state.use_cases.articles_repo
        .get_article_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::not_found("article"))?;

    state.use_cases.articles_repo.unfavorite_article(user.id, article.id).await?;

    let updated = state.use_cases.articles_repo
        .get_article_by_id(article.id)
        .await?
        .unwrap();

    let author = state.use_cases.users_repo
        .get_user_by_id(updated.author_id)
        .await?
        .ok_or_else(|| ApiError::not_found("author"))?;
    let following = state.use_cases.users_repo
        .is_following(user.id, author.id)
        .await
        .unwrap_or(false);
    let profile = author.to_profile(following);
    let favorited = state.use_cases.articles_repo
        .is_favorited(user.id, updated.id)
        .await
        .unwrap_or(false);
    let view = updated.to_view(profile, favorited);
    Ok(Json(ArticleEnvelope::from(view)))
}

async fn list_comments<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Path(slug): Path<String>,
    current_user: Option<CurrentUser>,
) -> ApiResult<Json<CommentsEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let article = state.use_cases.articles_repo
        .get_article_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::not_found("article"))?;
    let viewer_id = current_user.as_ref().map(|current| current.user.id);

    let mut comments = state.use_cases.comments_repo
        .get_comments_by_article(article.id)
        .await?;
    comments.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut views = Vec::with_capacity(comments.len());
    for comment in comments {
        let author = state.use_cases.users_repo
            .get_user_by_id(comment.author_id)
            .await?
            .ok_or_else(|| ApiError::not_found("author"))?;
        let following = if let Some(viewer) = viewer_id {
            state.use_cases.users_repo
                .is_following(viewer, author.id)
                .await
                .unwrap_or(false)
        } else {
            false
        };
        let profile = author.to_profile(following);
        views.push(comment.to_view(profile));
    }

    Ok(Json(CommentsEnvelope::from(views)))
}

async fn create_comment<U, A, C>(
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
    let article = state.use_cases.articles_repo
        .get_article_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::not_found("article"))?;

    let draft = CommentDraft::new(req.comment.body)?;
    let comment = Comment::new(
        CommentId::new(0), // DB will generate proper ID
        article.id,
        user.id,
        draft,
        Utc::now()
    );

    let created = state.use_cases.comments_repo.create_comment(comment).await?;

    let profile = user.to_profile(false);
    let view = created.to_view(profile);
    Ok(Json(CommentEnvelope::from(view)))
}

async fn delete_comment<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, .. }: CurrentUser,
    Path((slug, id)): Path<(String, i64)>,
) -> ApiResult<()>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let article = state.use_cases.articles_repo
        .get_article_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::not_found("article"))?;

    let comment_id = CommentId::new(id);
    let comment = state.use_cases.comments_repo
        .get_comment_by_id(comment_id)
        .await?
        .ok_or_else(|| ApiError::not_found("comment"))?;

    if comment.article_id != article.id {
        return Err(ApiError::not_found("comment"));
    }

    if comment.author_id != user.id {
        return Err(ApiError::not_found("comment"));
    }

    state.use_cases.comments_repo.delete_comment(comment_id).await?;
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

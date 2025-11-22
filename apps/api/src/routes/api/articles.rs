use std::collections::{HashMap, HashSet};

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post},
};
use chrono::Utc;
use domain::{
    Article, ArticleChanges, ArticleDraft, ArticleEnvelope, ArticleFilters, ArticleId,
    ArticlesEnvelope, Comment, CommentDraft, CommentEnvelope, CommentId, CommentsEnvelope,
    FeedFilters, Pagination, Profile, TagList, User, UserId,
};
use serde::Deserialize;

use crate::{
    auth::CurrentUser,
    error::{ApiError, ApiResult},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
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

async fn list_articles(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
    current_user: Option<CurrentUser>,
) -> ApiResult<Json<ArticlesEnvelope>> {
    let pagination = Pagination::new(query.limit, query.offset)?;
    let filters = ArticleFilters::new(query.tag, query.author, query.favorited, Some(pagination))?;

    let result = state.list_articles(filters).await;
    let users = state.users.read().await.clone();
    let followers = state.followers.read().await.clone();
    let favorites = state.favorites.read().await.clone();
    let viewer_id = current_user.as_ref().map(|current| current.user.id);

    let mut summaries = Vec::with_capacity(result.articles.len());
    for article in result.articles {
        let profile =
            author_profile_with_follow_state(&users, &followers, article.author_id, viewer_id)?;
        let favorited = viewer_id
            .map(|viewer| is_favorited(&favorites, article.id, viewer))
            .unwrap_or(false);
        summaries.push(article.to_summary(profile, favorited));
    }

    Ok(Json(ArticlesEnvelope {
        articles: summaries,
        articles_count: result.total,
    }))
}

async fn feed_articles(
    State(state): State<AppState>,
    Query(query): Query<FeedQuery>,
    CurrentUser { user, .. }: CurrentUser,
) -> ApiResult<Json<ArticlesEnvelope>> {
    let pagination = Pagination::new(query.limit, query.offset)?;
    let filters = FeedFilters::new(Some(pagination));
    let result = state.feed_articles(user.id, filters).await;
    let users = state.users.read().await.clone();
    let followers = state.followers.read().await.clone();
    let favorites = state.favorites.read().await.clone();

    let mut summaries = Vec::with_capacity(result.articles.len());
    for article in result.articles {
        let profile =
            author_profile_with_follow_state(&users, &followers, article.author_id, Some(user.id))?;
        let favorited = is_favorited(&favorites, article.id, user.id);
        summaries.push(article.to_summary(profile, favorited));
    }

    Ok(Json(ArticlesEnvelope {
        articles: summaries,
        articles_count: result.total,
    }))
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

async fn create_article(
    State(state): State<AppState>,
    CurrentUser { user, .. }: CurrentUser,
    Json(req): Json<CreateArticleRequest>,
) -> ApiResult<Json<ArticleEnvelope>> {
    let ArticlePayload {
        title,
        description,
        body,
        tag_list,
    } = req.article;

    let tags = TagList::new(tag_list)?;
    let draft = ArticleDraft::new(title, description, body, tags)?;

    let article = Article::publish(ArticleId::random(), user.id, draft, Utc::now())?;

    state.articles.write().await.push(article.clone());

    let profile = user.to_profile(false);
    let view = article.to_view(profile, false);
    Ok(Json(ArticleEnvelope::from(view)))
}

async fn get_article(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    current_user: Option<CurrentUser>,
) -> ApiResult<Json<ArticleEnvelope>> {
    let articles = state.articles.read().await;
    let article = articles
        .iter()
        .find(|candidate| candidate.slug.as_str() == slug)
        .cloned()
        .ok_or_else(|| ApiError::not_found("article"))?;
    drop(articles);

    let viewer_id = current_user.as_ref().map(|current| current.user.id);
    let author = author_profile_with_viewer(&state, article.author_id, viewer_id).await?;
    let favorited = match viewer_id {
        Some(viewer) => article_favorited_by(&state, article.id, viewer).await,
        None => false,
    };
    let view = article.to_view(author, favorited);
    Ok(Json(ArticleEnvelope::from(view)))
}

async fn update_article(
    State(state): State<AppState>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
    Json(req): Json<UpdateArticleRequest>,
) -> ApiResult<Json<ArticleEnvelope>> {
    let UpdateArticlePayload {
        title,
        description,
        body,
        tag_list,
    } = req.article;

    let updated = {
        let mut articles = state.articles.write().await;
        let Some(article) = articles
            .iter_mut()
            .find(|candidate| candidate.slug.as_str() == slug)
        else {
            return Err(ApiError::not_found("article"));
        };

        if article.author_id != user.id {
            return Err(ApiError::unauthorized("cannot edit another user's article"));
        }

        let mut changes = ArticleChanges::default();
        changes.title = title;
        changes.description = description;
        changes.body = body;
        if let Some(tags) = tag_list {
            changes.tag_list = Some(TagList::new(tags)?);
        }

        article.apply_changes(changes, Utc::now())?;
        article.clone()
    };

    let favorited = article_favorited_by(&state, updated.id, user.id).await;
    let profile = user.to_profile(false);
    let view = updated.to_view(profile, favorited);
    Ok(Json(ArticleEnvelope::from(view)))
}

async fn delete_article(
    State(state): State<AppState>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
) -> ApiResult<()> {
    let removed_id = {
        let mut articles = state.articles.write().await;
        if let Some(pos) = articles
            .iter()
            .position(|candidate| candidate.slug.as_str() == slug && candidate.author_id == user.id)
        {
            articles.remove(pos).id
        } else {
            return Err(ApiError::not_found("article"));
        }
    };

    state.favorites.write().await.remove(&removed_id);
    Ok(())
}

async fn favorite_article(
    State(state): State<AppState>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
) -> ApiResult<Json<ArticleEnvelope>> {
    let (article_id, author_id) = article_identifiers(&state, &slug).await?;

    let mut favorites = state.favorites.write().await;
    let entry = favorites.entry(article_id).or_insert_with(HashSet::new);
    let inserted = entry.insert(user.id);
    drop(favorites);

    let updated = {
        let mut articles = state.articles.write().await;
        let Some(article) = articles
            .iter_mut()
            .find(|candidate| candidate.slug.as_str() == slug)
        else {
            return Err(ApiError::not_found("article"));
        };
        if inserted {
            article.favorite();
        }
        article.clone()
    };

    let author = author_profile_with_viewer(&state, author_id, Some(user.id)).await?;
    let view = updated.to_view(author, true);
    Ok(Json(ArticleEnvelope::from(view)))
}

async fn unfavorite_article(
    State(state): State<AppState>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
) -> ApiResult<Json<ArticleEnvelope>> {
    let (article_id, author_id) = article_identifiers(&state, &slug).await?;

    let mut favorites = state.favorites.write().await;
    let removed = favorites
        .get_mut(&article_id)
        .map(|users| users.remove(&user.id))
        .unwrap_or(false);
    drop(favorites);

    let updated = {
        let mut articles = state.articles.write().await;
        let Some(article) = articles
            .iter_mut()
            .find(|candidate| candidate.slug.as_str() == slug)
        else {
            return Err(ApiError::not_found("article"));
        };
        if removed {
            article.unfavorite();
        }
        article.clone()
    };

    let author = author_profile_with_viewer(&state, author_id, Some(user.id)).await?;
    let favorited = article_favorited_by(&state, updated.id, user.id).await;
    let view = updated.to_view(author, favorited);
    Ok(Json(ArticleEnvelope::from(view)))
}

async fn list_comments(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    current_user: Option<CurrentUser>,
) -> ApiResult<Json<CommentsEnvelope>> {
    let (article_id, _) = article_identifiers(&state, &slug).await?;
    let viewer_id = current_user.as_ref().map(|current| current.user.id);

    let mut comments = state
        .comments
        .read()
        .await
        .iter()
        .filter(|comment| comment.article_id == article_id)
        .cloned()
        .collect::<Vec<Comment>>();
    comments.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let users = state.users.read().await.clone();
    let followers = state.followers.read().await.clone();
    let mut views = Vec::with_capacity(comments.len());
    for comment in comments {
        let author =
            author_profile_with_follow_state(&users, &followers, comment.author_id, viewer_id)?;
        views.push(comment.to_view(author));
    }

    Ok(Json(CommentsEnvelope::from(views)))
}

async fn create_comment(
    State(state): State<AppState>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
    Json(req): Json<CreateCommentRequest>,
) -> ApiResult<Json<CommentEnvelope>> {
    let (article_id, _) = article_identifiers(&state, &slug).await?;

    let draft = CommentDraft::new(req.comment.body)?;
    let comment = {
        let mut next_id = state.next_comment_id.write().await;
        let comment_id = CommentId::new(*next_id);
        *next_id = next_id.saturating_add(1);
        Comment::new(comment_id, article_id, user.id, draft, Utc::now())
    };

    state.comments.write().await.push(comment.clone());

    let author = author_profile_with_viewer(&state, user.id, Some(user.id)).await?;
    let view = comment.to_view(author);
    Ok(Json(CommentEnvelope::from(view)))
}

async fn delete_comment(
    State(state): State<AppState>,
    CurrentUser { user, .. }: CurrentUser,
    Path((slug, id)): Path<(String, i64)>,
) -> ApiResult<()> {
    let (article_id, _) = article_identifiers(&state, &slug).await?;

    let mut comments = state.comments.write().await;
    if let Some(pos) = comments
        .iter()
        .position(|comment| comment.article_id == article_id && comment.id.as_i64() == id)
    {
        if comments[pos].author_id != user.id {
            return Err(ApiError::unauthorized(
                "cannot delete another user's comment",
            ));
        }
        comments.remove(pos);
        Ok(())
    } else {
        Err(ApiError::not_found("comment"))
    }
}

async fn article_identifiers(state: &AppState, slug: &str) -> ApiResult<(ArticleId, UserId)> {
    let articles = state.articles.read().await;
    articles
        .iter()
        .find(|candidate| candidate.slug.as_str() == slug)
        .map(|article| (article.id, article.author_id))
        .ok_or_else(|| ApiError::not_found("article"))
}

fn author_profile_with_follow_state(
    users: &[User],
    followers: &HashMap<UserId, HashSet<UserId>>,
    author_id: UserId,
    viewer_id: Option<UserId>,
) -> ApiResult<Profile> {
    let user = users
        .iter()
        .find(|candidate| candidate.id == author_id)
        .cloned()
        .ok_or_else(|| ApiError::not_found("author"))?;

    let following = viewer_id
        .and_then(|viewer| followers.get(&author_id).map(|set| set.contains(&viewer)))
        .unwrap_or(false);

    Ok(user.to_profile(following))
}

async fn author_profile_with_viewer(
    state: &AppState,
    author_id: UserId,
    viewer_id: Option<UserId>,
) -> ApiResult<Profile> {
    let followers = state.followers.read().await.clone();
    let users = state.users.read().await.clone();
    author_profile_with_follow_state(&users, &followers, author_id, viewer_id)
}

fn is_favorited(
    favorites: &HashMap<ArticleId, HashSet<UserId>>,
    article_id: ArticleId,
    user_id: UserId,
) -> bool {
    favorites
        .get(&article_id)
        .map(|users| users.contains(&user_id))
        .unwrap_or(false)
}

async fn article_favorited_by(state: &AppState, article_id: ArticleId, user_id: UserId) -> bool {
    state
        .favorites
        .read()
        .await
        .get(&article_id)
        .map(|users| users.contains(&user_id))
        .unwrap_or(false)
}

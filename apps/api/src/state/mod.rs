use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use domain::{
    Article, ArticleFilters, ArticleId, Comment, FeedFilters, Tag, TagList, User, UserId,
};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub tags: Arc<RwLock<TagList>>, // placeholder store until DB layer arrives
    pub users: Arc<RwLock<Vec<User>>>,
    pub sessions: Arc<RwLock<HashMap<String, UserId>>>,
    pub articles: Arc<RwLock<Vec<Article>>>,
    pub favorites: Arc<RwLock<HashMap<ArticleId, HashSet<UserId>>>>,
    pub comments: Arc<RwLock<Vec<Comment>>>,
    pub next_comment_id: Arc<RwLock<i64>>,
    pub followers: Arc<RwLock<HashMap<UserId, HashSet<UserId>>>>,
}

pub struct ArticleListResult {
    pub articles: Vec<Article>,
    pub total: usize,
}

impl Default for AppState {
    fn default() -> Self {
        let mut tags = TagList::default();
        for label in ["rust", "axum", "realworld"] {
            if let Ok(tag) = Tag::new(label) {
                tags.push(tag);
            }
        }

        Self {
            tags: Arc::new(RwLock::new(tags)),
            users: Arc::new(RwLock::new(Vec::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            articles: Arc::new(RwLock::new(Vec::new())),
            favorites: Arc::new(RwLock::new(HashMap::new())),
            comments: Arc::new(RwLock::new(Vec::new())),
            next_comment_id: Arc::new(RwLock::new(1)),
            followers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl AppState {
    pub async fn list_articles(&self, filters: ArticleFilters) -> ArticleListResult {
        let articles = self.articles.read().await.clone();
        let users = self.users.read().await.clone();
        let favorites = self.favorites.read().await.clone();

        let favorited_user = filters
            .favorited
            .as_deref()
            .and_then(|username| find_user_id_by_username(&users, username));

        let mut filtered = Vec::new();
        for article in articles {
            if let Some(tag) = filters.tag.as_ref() {
                if !article.tag_list.contains(tag) {
                    continue;
                }
            }
            if let Some(author) = filters.author.as_deref() {
                if !is_author(&users, article.author_id, author) {
                    continue;
                }
            }
            if let Some(user_id) = favorited_user {
                if !is_favorited(&favorites, article.id, user_id) {
                    continue;
                }
            }
            filtered.push(article);
        }

        filtered.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        paginate(
            filtered,
            filters.pagination.limit(),
            filters.pagination.offset(),
        )
    }

    pub async fn feed_articles(
        &self,
        follower_id: UserId,
        filters: FeedFilters,
    ) -> ArticleListResult {
        let articles = self.articles.read().await.clone();
        let followers = self.followers.read().await.clone();

        let following: HashSet<UserId> = followers
            .iter()
            .filter_map(|(author_id, subscribers)| {
                if subscribers.contains(&follower_id) {
                    Some(*author_id)
                } else {
                    None
                }
            })
            .collect();

        let mut feed: Vec<Article> = articles
            .into_iter()
            .filter(|article| following.contains(&article.author_id))
            .collect();

        feed.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        paginate(
            feed,
            filters.pagination.limit(),
            filters.pagination.offset(),
        )
    }
}

fn paginate(articles: Vec<Article>, limit: u32, offset: u32) -> ArticleListResult {
    let total = articles.len();
    let start = offset as usize;
    let end = (start + limit as usize).min(total);
    let slice = if start >= total {
        Vec::new()
    } else {
        articles[start..end].to_vec()
    };

    ArticleListResult {
        articles: slice,
        total,
    }
}

fn find_user_id_by_username(users: &[User], username: &str) -> Option<UserId> {
    users
        .iter()
        .find(|candidate| candidate.username.as_str() == username)
        .map(|user| user.id)
}

fn is_author(users: &[User], author_id: UserId, username: &str) -> bool {
    users
        .iter()
        .find(|candidate| candidate.id == author_id)
        .map(|user| user.username.as_str() == username)
        .unwrap_or(false)
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

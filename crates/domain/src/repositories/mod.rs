pub mod in_memory;

use async_trait::async_trait;
use crate::{
    Article, ArticleId, ArticleFilters, ArticlesEnvelope, FeedFilters,
    Comment, CommentId, User, UserId,
};

pub use in_memory::{InMemoryArticlesRepository, InMemoryCommentsRepository, InMemoryUsersRepository};


#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<User>>;
    async fn get_user_by_id(&self, id: UserId) -> anyhow::Result<Option<User>>;
    async fn create_user(&self, user: User) -> anyhow::Result<User>;
    async fn update_user(&self, user: User) -> anyhow::Result<User>;
    async fn follow_user(&self, follower_id: UserId, followee_id: UserId) -> anyhow::Result<()>;
    async fn unfollow_user(&self, follower_id: UserId, followee_id: UserId) -> anyhow::Result<()>;
    async fn is_following(&self, follower_id: UserId, followee_id: UserId) -> anyhow::Result<bool>;
}

#[async_trait]
pub trait ArticlesRepository: Send + Sync {
    async fn create_article(&self, article: Article) -> anyhow::Result<Article>;
    async fn get_article_by_slug(&self, slug: &str) -> anyhow::Result<Option<Article>>;
    async fn get_article_by_id(&self, id: ArticleId) -> anyhow::Result<Option<Article>>;
    async fn update_article(&self, article: Article) -> anyhow::Result<Article>;
    async fn delete_article(&self, id: ArticleId) -> anyhow::Result<()>;
    async fn list_articles(&self, filters: ArticleFilters) -> anyhow::Result<ArticlesEnvelope>;
    async fn feed_articles(&self, user_id: UserId, filters: FeedFilters) -> anyhow::Result<ArticlesEnvelope>;
    async fn favorite_article(&self, user_id: UserId, article_id: ArticleId) -> anyhow::Result<()>;
    async fn unfavorite_article(&self, user_id: UserId, article_id: ArticleId) -> anyhow::Result<()>;
    async fn is_favorited(&self, user_id: UserId, article_id: ArticleId) -> anyhow::Result<bool>;
}

#[async_trait]
pub trait CommentsRepository: Send + Sync {
    async fn create_comment(&self, comment: Comment) -> anyhow::Result<Comment>;
    async fn get_comments_by_article(&self, article_id: ArticleId) -> anyhow::Result<Vec<Comment>>;
    async fn delete_comment(&self, id: CommentId) -> anyhow::Result<()>;
    async fn get_comment_by_id(&self, id: CommentId) -> anyhow::Result<Option<Comment>>;
}

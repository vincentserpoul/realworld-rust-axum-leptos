use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    Article, ArticleFilters, ArticleId, ArticlesEnvelope,
    Comment, CommentId, FeedFilters, User, UserId,
    services::{add_follower, is_following, remove_follower, is_article_favorited},
    repositories::{ArticlesRepository, CommentsRepository, UsersRepository},
};

#[derive(Clone, Default)]
pub struct InMemoryUsersRepository {
    users: Arc<RwLock<Vec<User>>>,
    followers: Arc<RwLock<HashMap<UserId, std::collections::HashSet<UserId>>>>,
}

impl InMemoryUsersRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl UsersRepository for InMemoryUsersRepository {
    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let users = self.users.read().await;
        Ok(users.iter().find(|u| u.email.as_str() == email).cloned())
    }

    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        let users = self.users.read().await;
        Ok(users.iter().find(|u| u.username.as_str() == username).cloned())
    }

    async fn get_user_by_id(&self, id: UserId) -> anyhow::Result<Option<User>> {
        let users = self.users.read().await;
        Ok(users.iter().find(|u| u.id == id).cloned())
    }

    async fn create_user(&self, user: User) -> anyhow::Result<User> {
        let mut users = self.users.write().await;
        users.push(user.clone());
        Ok(user)
    }

    async fn update_user(&self, user: User) -> anyhow::Result<User> {
        let mut users = self.users.write().await;
        if let Some(existing) = users.iter_mut().find(|u| u.id == user.id) {
            *existing = user.clone();
            Ok(user)
        } else {
            Err(anyhow::anyhow!("User not found"))
        }
    }

    async fn follow_user(&self, follower_id: UserId, followee_id: UserId) -> anyhow::Result<()> {
        let mut followers = self.followers.write().await;
        add_follower(&mut followers, followee_id, follower_id);
        Ok(())
    }

    async fn unfollow_user(&self, follower_id: UserId, followee_id: UserId) -> anyhow::Result<()> {
        let mut followers = self.followers.write().await;
        remove_follower(&mut followers, followee_id, follower_id);
        Ok(())
    }

    async fn is_following(&self, follower_id: UserId, followee_id: UserId) -> anyhow::Result<bool> {
        let followers = self.followers.read().await;
        Ok(is_following(&followers, followee_id, follower_id))
    }
}

#[derive(Clone)]
pub struct InMemoryArticlesRepository {
    articles: Arc<RwLock<Vec<Article>>>,
    favorites: Arc<RwLock<HashMap<ArticleId, std::collections::HashSet<UserId>>>>,
    users_repo: InMemoryUsersRepository,
}

impl InMemoryArticlesRepository {
    pub fn new(users_repo: InMemoryUsersRepository) -> Self {
        Self {
            articles: Arc::new(RwLock::new(Vec::new())),
            favorites: Arc::new(RwLock::new(HashMap::new())),
            users_repo,
        }
    }
}

#[async_trait]
impl ArticlesRepository for InMemoryArticlesRepository {
    async fn create_article(&self, article: Article) -> anyhow::Result<Article> {
        let mut articles = self.articles.write().await;
        articles.push(article.clone());
        Ok(article)
    }

    async fn get_article_by_slug(&self, slug: &str) -> anyhow::Result<Option<Article>> {
        let articles = self.articles.read().await;
        Ok(Article::find_by_slug_owned(&articles, slug))
    }

    async fn get_article_by_id(&self, id: ArticleId) -> anyhow::Result<Option<Article>> {
        let articles = self.articles.read().await;
        Ok(articles.iter().find(|a| a.id == id).cloned())
    }

    async fn update_article(&self, article: Article) -> anyhow::Result<Article> {
        let mut articles = self.articles.write().await;
        if let Some(existing) = articles.iter_mut().find(|a| a.id == article.id) {
            *existing = article.clone();
            Ok(article)
        } else {
            Err(anyhow::anyhow!("Article not found"))
        }
    }

    async fn delete_article(&self, id: ArticleId) -> anyhow::Result<()> {
        let mut articles = self.articles.write().await;
        articles.retain(|a| a.id != id);
        Ok(())
    }

    async fn list_articles(&self, filters: ArticleFilters) -> anyhow::Result<ArticlesEnvelope> {
        let articles = self.articles.read().await;
        let users = self.users_repo.users.read().await;
        let favorites = self.favorites.read().await;

        // Filter articles
        let mut filtered: Vec<&Article> = articles.iter().collect();

        if let Some(ref tag) = filters.tag {
            filtered.retain(|a| a.tag_list.contains(tag));
        }

        if let Some(ref author_username) = filters.author {
            if let Some(author) = users.iter().find(|u| u.username.as_str() == author_username) {
                filtered.retain(|a| a.author_id == author.id);
            } else {
                filtered.clear();
            }
        }

        if let Some(ref favorited_username) = filters.favorited {
            if let Some(user) = users.iter().find(|u| u.username.as_str() == favorited_username) {
                filtered.retain(|a| is_article_favorited(&favorites, a.id, user.id));
            } else {
                filtered.clear();
            }
        }

        let total = filtered.len();

        // Apply pagination
        let start = filters.pagination.offset() as usize;
        let end = (start + filters.pagination.limit() as usize).min(total);
        let paginated: Vec<&Article> = filtered.into_iter().skip(start).take(end - start).collect();

        // Build summaries
        let mut summaries = Vec::new();
        for article in paginated {
            if let Some(author) = users.iter().find(|u| u.id == article.author_id) {
                let following = false; // TODO: needs viewer context
                let favorited = false; // TODO: needs viewer context
                let profile = author.to_profile(following);
                summaries.push(article.to_summary(profile, favorited));
            }
        }

        Ok(ArticlesEnvelope {
            articles: summaries,
            articles_count: total,
        })
    }

    async fn feed_articles(&self, user_id: UserId, filters: FeedFilters) -> anyhow::Result<ArticlesEnvelope> {
        let articles = self.articles.read().await;
        let users = self.users_repo.users.read().await;
        let followers = self.users_repo.followers.read().await;
        let favorites = self.favorites.read().await;

        // Get all users that this user follows
        let mut followed_users = Vec::new();
        for (target_id, follower_set) in followers.iter() {
            if follower_set.contains(&user_id) {
                followed_users.push(*target_id);
            }
        }

        // Filter articles by followed authors
        let filtered: Vec<&Article> = articles
            .iter()
            .filter(|a| followed_users.contains(&a.author_id))
            .collect();

        let total = filtered.len();

        // Apply pagination
        let start = filters.pagination.offset() as usize;
        let end = (start + filters.pagination.limit() as usize).min(total);
        let paginated: Vec<&Article> = filtered.into_iter().skip(start).take(end - start).collect();

        // Build summaries
        let mut summaries = Vec::new();
        for article in paginated {
            if let Some(author) = users.iter().find(|u| u.id == article.author_id) {
                let following = true; // By definition
                let favorited = is_article_favorited(&favorites, article.id, user_id);
                let profile = author.to_profile(following);
                summaries.push(article.to_summary(profile, favorited));
            }
        }

        Ok(ArticlesEnvelope {
            articles: summaries,
            articles_count: total,
        })
    }

    async fn favorite_article(&self, user_id: UserId, article_id: ArticleId) -> anyhow::Result<()> {
        let mut favorites = self.favorites.write().await;
        let entry = favorites.entry(article_id).or_insert_with(std::collections::HashSet::new);
        entry.insert(user_id);

        // Update favorites count
        let mut articles = self.articles.write().await;
        if let Some(article) = articles.iter_mut().find(|a| a.id == article_id) {
            article.favorites_count += 1;
        }

        Ok(())
    }

    async fn unfavorite_article(&self, user_id: UserId, article_id: ArticleId) -> anyhow::Result<()> {
        let mut favorites = self.favorites.write().await;
        if let Some(entry) = favorites.get_mut(&article_id) {
            entry.remove(&user_id);
        }

        // Update favorites count
        let mut articles = self.articles.write().await;
        if let Some(article) = articles.iter_mut().find(|a| a.id == article_id) {
            article.favorites_count = article.favorites_count.saturating_sub(1);
        }

        Ok(())
    }

    async fn is_favorited(&self, user_id: UserId, article_id: ArticleId) -> anyhow::Result<bool> {
        let favorites = self.favorites.read().await;
        Ok(is_article_favorited(&favorites, article_id, user_id))
    }
}

#[derive(Clone, Default)]
pub struct InMemoryCommentsRepository {
    comments: Arc<RwLock<Vec<Comment>>>,
}

impl InMemoryCommentsRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl CommentsRepository for InMemoryCommentsRepository {
    async fn create_comment(&self, comment: Comment) -> anyhow::Result<Comment> {
        let mut comments = self.comments.write().await;
        comments.push(comment.clone());
        Ok(comment)
    }

    async fn get_comments_by_article(&self, article_id: ArticleId) -> anyhow::Result<Vec<Comment>> {
        let comments = self.comments.read().await;
        Ok(comments
            .iter()
            .filter(|c| c.article_id == article_id)
            .cloned()
            .collect())
    }

    async fn delete_comment(&self, id: CommentId) -> anyhow::Result<()> {
        let mut comments = self.comments.write().await;
        comments.retain(|c| c.id != id);
        Ok(())
    }

    async fn get_comment_by_id(&self, id: CommentId) -> anyhow::Result<Option<Comment>> {
        let comments = self.comments.read().await;
        Ok(comments.iter().find(|c| c.id == id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Email, PasswordHash, Username};

    fn create_test_user(id: UserId, username: &str, email: &str) -> User {
        User::new(
            id,
            Email::parse(email).unwrap(),
            Username::new(username).unwrap(),
            PasswordHash::new("hash".to_string()).unwrap(),
            chrono::Utc::now(),
        )
    }

    #[tokio::test]
    async fn test_create_and_get_user() {
        let repo = InMemoryUsersRepository::new();
        let user = create_test_user(UserId::random(), "testuser", "test@example.com");

        repo.create_user(user.clone()).await.unwrap();
        
        let found = repo.get_user_by_email("test@example.com").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().username.as_str(), "testuser");
    }

    #[tokio::test]
    async fn test_follow_and_is_following() {
        let repo = InMemoryUsersRepository::new();
        let user1_id = UserId::random();
        let user2_id = UserId::random();

        repo.follow_user(user1_id, user2_id).await.unwrap();
        
        let is_following = repo.is_following(user1_id, user2_id).await.unwrap();
        assert!(is_following);
    }

    #[tokio::test]
    async fn test_create_and_get_article() {
        let users_repo = InMemoryUsersRepository::new();
        let repo = InMemoryArticlesRepository::new(users_repo);
        
        let article_id = ArticleId::random();
        let author_id = UserId::random();
        
        let article = Article {
            id: article_id,
            author_id,
            slug: crate::Slug::new("test-slug").unwrap(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            body: "Test".to_string(),
            tag_list: crate::TagList::default(),
            favorites_count: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        repo.create_article(article.clone()).await.unwrap();
        
        let found = repo.get_article_by_slug("test-slug").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "Test");
    }

    #[tokio::test]
    async fn test_favorite_article() {
        let users_repo = InMemoryUsersRepository::new();
        let repo = InMemoryArticlesRepository::new(users_repo);
        
        let article_id = ArticleId::random();
        let user_id = UserId::random();
        
        let article = Article {
            id: article_id,
            author_id: UserId::random(),
            slug: crate::Slug::new("test-slug").unwrap(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            body: "Test".to_string(),
            tag_list: crate::TagList::default(),
            favorites_count: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        repo.create_article(article).await.unwrap();
        repo.favorite_article(user_id, article_id).await.unwrap();
        
        let is_favorited = repo.is_favorited(user_id, article_id).await.unwrap();
        assert!(is_favorited);
    }
}

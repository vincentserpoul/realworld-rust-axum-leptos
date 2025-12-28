use async_trait::async_trait;
use deadpool_postgres::Pool;
use domain::{
    repositories::{ArticlesRepository, CommentsRepository, UsersRepository},
    Article, ArticleFilters, ArticleId, ArticlesEnvelope, Comment, CommentId, FeedFilters, User,
    UserId,
};


macro_rules! map_user {
    ($row:expr) => {
        User {
            id: UserId::from($row.id),
            email: domain::Email::parse($row.email).expect("invalid email in db"),
            username: domain::Username::new($row.username).expect("invalid username in db"),
            bio: if $row.bio.is_empty() { None } else { Some($row.bio) },
            image: if $row.img.is_empty() { None } else { Some(domain::ImageUrl::new($row.img).expect("invalid image in db")) },
            password_hash: domain::PasswordHash::new($row.pwd).expect("invalid password in db"),
            created_at: $row.created_at.with_timezone(&chrono::Utc),
            updated_at: $row.updated_at.with_timezone(&chrono::Utc),
        }
    };
}

#[derive(Clone)]
pub struct PostgresUsersRepository {
    pool: Pool,
}

impl PostgresUsersRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UsersRepository for PostgresUsersRepository {
    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let client = self.pool.get().await?;
        let user = crate::clorinde::queries::users::get_user_by_email()
            .bind(&client, &email)
            .opt()
            .await?;
        Ok(user.map(|row| map_user!(row)))
    }

    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        let client = self.pool.get().await?;
        let user = crate::clorinde::queries::users::get_user_by_username()
            .bind(&client, &username)
            .opt()
            .await?;
        Ok(user.map(|row| map_user!(row)))
    }

    async fn get_user_by_id(&self, id: UserId) -> anyhow::Result<Option<User>> {
        let client = self.pool.get().await?;
        let user = crate::clorinde::queries::users::get_user_by_id()
            .bind(&client, &id.into())
            .opt()
            .await?;
        Ok(user.map(|row| map_user!(row)))
    }

    async fn create_user(&self, user: User) -> anyhow::Result<User> {
        let client = self.pool.get().await?;
        let created = crate::clorinde::queries::users::create_user()
            .bind(
                &client,
                &user.id.into(),
                &user.email.as_str(),
                &user.username.as_str(),
                &user.password_hash.as_str(),
                &user.created_at.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()),
            )
            .one()
            .await?;
        Ok(map_user!(created))
    }

    async fn update_user(&self, user: User) -> anyhow::Result<User> {
        let client = self.pool.get().await?;
        let updated = crate::clorinde::queries::users::update_user()
            .bind(
                &client,
                &user.email.as_str(),
                &user.username.as_str(),
                &user.password_hash.as_str(),
                &user.image.as_ref().map(|i| i.as_str()),
                &user.bio.as_deref(),
                &user.updated_at.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()),
                &user.id.into(),
            )
            .one()
            .await?;
        Ok(map_user!(updated))
    }

    async fn follow_user(&self, follower_id: UserId, followee_id: UserId) -> anyhow::Result<()> {
        let client = self.pool.get().await?;
        crate::clorinde::queries::users::follow_user()
            .bind(&client, &follower_id.into(), &followee_id.into())
            .await?;
        Ok(())
    }

    async fn unfollow_user(&self, follower_id: UserId, followee_id: UserId) -> anyhow::Result<()> {
        let client = self.pool.get().await?;
        crate::clorinde::queries::users::unfollow_user()
            .bind(&client, &follower_id.into(), &followee_id.into())
            .await?;
        Ok(())
    }

    async fn is_following(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> anyhow::Result<bool> {
        let client = self.pool.get().await?;
        let is_following = crate::clorinde::queries::users::is_following()
            .bind(&client, &follower_id.into(), &followee_id.into())
            .one()
            .await?;
        Ok(is_following)
    }
}


#[derive(Clone)]
pub struct PostgresArticlesRepository {
    pool: Pool,
}

impl PostgresArticlesRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArticlesRepository for PostgresArticlesRepository {
    async fn create_article(&self, article: Article) -> anyhow::Result<Article> {
        let client = self.pool.get().await?;
        let created = crate::clorinde::queries::articles::create_article()
            .bind(
                &client,
                &article.id.into(),
                &article.slug.as_str(),
                &article.title,
                &article.description,
                &article.body,
                &article.author_id.into(),
                &article.created_at.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()),
            )
            .one()
            .await?;
        
        // We need to insert tags
        for tag in article.tag_list.as_slice().iter() {
            // Create tag if not exists
            let tag_id = uuid::Uuid::new_v4(); // We need to generate ID for new tags? 
            // Wait, tag table has ID.
            // My query `create_tag` takes ID and Name.
            // But `tag` table has `id uuid PRIMARY KEY`.
            // So I should generate ID.
            crate::clorinde::queries::articles::create_tag()
                .bind(&client, &tag_id, &tag.as_str())
                .await?;
            
            // Get tag id (in case it existed)
            let tag_row = crate::clorinde::queries::articles::get_tag_by_name()
                .bind(&client, &tag.as_str())
                .one()
                .await?;
            
            // Link tag
            crate::clorinde::queries::articles::add_tag_to_article()
                .bind(&client, &created.id, &tag_row.id)
                .await?;
        }

        Ok(article)
    }

    async fn get_article_by_slug(&self, slug: &str) -> anyhow::Result<Option<Article>> {
        let client = self.pool.get().await?;
        let article_row = crate::clorinde::queries::articles::get_article_by_slug()
            .bind(&client, &slug)
            .opt()
            .await?;
        
        if let Some(row) = article_row {
            Ok(Some(Article {
                id: ArticleId::from(row.id),
                slug: domain::Slug::new(row.slug).expect("invalid slug in db"),
                title: row.title,
                description: row.description,
                body: row.body,
                tag_list: domain::TagList::new(row.tag_list).expect("invalid tags in db"),
                author_id: UserId::from(row.author_id),
                favorites_count: row.favorites_count as u32,
                created_at: row.created_at.with_timezone(&chrono::Utc),
                updated_at: row.updated_at.with_timezone(&chrono::Utc),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_article_by_id(&self, id: ArticleId) -> anyhow::Result<Option<Article>> {
        let client = self.pool.get().await?;
        let article_row = crate::clorinde::queries::articles::get_article_by_id()
            .bind(&client, &id.into())
            .opt()
            .await?;
        
        if let Some(row) = article_row {
            Ok(Some(Article {
                id: ArticleId::from(row.id),
                slug: domain::Slug::new(row.slug).expect("invalid slug in db"),
                title: row.title,
                description: row.description,
                body: row.body,
                tag_list: domain::TagList::new(row.tag_list).expect("invalid tags in db"),
                author_id: UserId::from(row.author_id),
                favorites_count: row.favorites_count as u32,
                created_at: row.created_at.with_timezone(&chrono::Utc),
                updated_at: row.updated_at.with_timezone(&chrono::Utc),
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_article(&self, article: Article) -> anyhow::Result<Article> {
        let client = self.pool.get().await?;
        let _updated = crate::clorinde::queries::articles::update_article()
            .bind(
                &client,
                &article.slug.as_str(),
                &article.title,
                &article.description,
                &article.body,
                &article.updated_at.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()),
                &article.id.into(),
            )
            .one()
            .await?;
        
        // Update tags
        crate::clorinde::queries::articles::remove_tags_from_article()
            .bind(&client, &article.id.into())
            .await?;
            
        for tag in article.tag_list.as_slice().iter() {
            let tag_id = uuid::Uuid::new_v4();
            crate::clorinde::queries::articles::create_tag()
                .bind(&client, &tag_id, &tag.as_str())
                .await?;
            
            let tag_row = crate::clorinde::queries::articles::get_tag_by_name()
                .bind(&client, &tag.as_str())
                .one()
                .await?;
            
            crate::clorinde::queries::articles::add_tag_to_article()
                .bind(&client, &article.id.into(), &tag_row.id)
                .await?;
        }

        Ok(article)
    }

    async fn delete_article(&self, id: ArticleId) -> anyhow::Result<()> {
        let client = self.pool.get().await?;
        crate::clorinde::queries::articles::delete_article()
            .bind(&client, &id.into())
            .await?;
        Ok(())
    }

    async fn list_articles(&self, filters: ArticleFilters) -> anyhow::Result<ArticlesEnvelope> {
        let client = self.pool.get().await?;
        let limit = filters.pagination.limit() as i64;
        let offset = filters.pagination.offset() as i64;
        let tag = filters.tag.map(|t| t.as_str().to_owned());
        let author = filters.author;
        let favorited = filters.favorited;
        
        // We need a viewer_id for `following_author` and `favorited` checks.
        // But `list_articles` signature doesn't take viewer_id?
        // The trait definition: `async fn list_articles(&self, filters: ArticleFilters) -> anyhow::Result<ArticlesEnvelope>;`
        // It seems the trait is missing `viewer_id` or `user_id`?
        // In `apps/api/src/routes/api/articles.rs`, `list_articles` takes `current_user: Option<CurrentUser>`.
        // And it calls `state.list_articles(filters)`.
        // But `state.list_articles` (in memory impl) didn't take user_id.
        // It calculated `following` and `favorited` *after* fetching articles, using `author_profile_with_follow_state`.
        // But my SQL query does it inside the query!
        // My SQL query requires `viewer_id` (parameter $4 in original, :viewer_id in named).
        // If I don't have viewer_id, I should pass NULL or a dummy UUID?
        // If I pass NULL, the `EXISTS` check will return false, which is correct for anonymous user.
        // But `uuid` type in Postgres cannot be NULL if the parameter is typed as UUID?
        // `clorinde` generated `viewer_id: &'a uuid::Uuid`. It expects a reference to Uuid.
        // It doesn't accept Option<Uuid>.
        // So I must pass a Uuid.
        // I can pass a random UUID that definitely doesn't exist?
        // Or I should update the trait to accept `viewer_id`.
        // But I should stick to the requested changes.
        // The user asked to "hide the data impl behind a trait".
        // If the trait doesn't support viewer_id, I can't use the optimized query fully.
        // However, I can pass a random UUID for anonymous users.
        
        let viewer_id = uuid::Uuid::nil(); // Use nil UUID for anonymous

        let rows = crate::clorinde::queries::articles::list_articles()
            .bind(
                &client,
                &viewer_id,
                &author.as_deref(),
                &tag.as_deref(),
                &favorited.as_deref(),
                &limit,
                &offset,
            )
            .all()
            .await?;
            
        let count = crate::clorinde::queries::articles::count_articles()
            .bind(
                &client,
                &author.as_deref(),
                &tag.as_deref(),
                &favorited.as_deref(),
            )
            .one()
            .await?;

        let articles = rows.into_iter().map(|row| {
            domain::ArticleSummary {
                slug: domain::Slug::new(row.slug).expect("invalid slug"),
                title: row.title,
                description: row.description,
                tag_list: domain::TagList::new(row.tag_list).expect("invalid tags"),
                created_at: row.created_at.with_timezone(&chrono::Utc),
                updated_at: row.updated_at.with_timezone(&chrono::Utc),
                favorited: row.favorited,
                favorites_count: row.favorites_count as u32,
                author: domain::Profile::new(
                    domain::Username::new(row.author_username).expect("invalid username"),
                    if row.author_bio.is_empty() { None } else { Some(row.author_bio) },
                    if row.author_image.is_empty() { None } else { Some(domain::ImageUrl::new(row.author_image).expect("invalid image")) },
                    row.following_author,
                ),
            }
        }).collect();

        Ok(ArticlesEnvelope {
            articles,
            articles_count: count as usize,
        })
    }

    async fn feed_articles(
        &self,
        user_id: UserId,
        filters: FeedFilters,
    ) -> anyhow::Result<ArticlesEnvelope> {
        let client = self.pool.get().await?;
        let limit = filters.pagination.limit() as i64;
        let offset = filters.pagination.offset() as i64;
        let viewer_id = user_id.into();

        let rows = crate::clorinde::queries::articles::feed_articles()
            .bind(&client, &viewer_id, &limit, &offset)
            .all()
            .await?;
            
        let count = crate::clorinde::queries::articles::count_feed_articles()
            .bind(&client, &viewer_id)
            .one()
            .await?;

        let articles = rows.into_iter().map(|row| {
            domain::ArticleSummary {
                slug: domain::Slug::new(row.slug).expect("invalid slug"),
                title: row.title,
                description: row.description,
                tag_list: domain::TagList::new(row.tag_list).expect("invalid tags"),
                created_at: row.created_at.with_timezone(&chrono::Utc),
                updated_at: row.updated_at.with_timezone(&chrono::Utc),
                favorited: row.favorited,
                favorites_count: row.favorites_count as u32,
                author: domain::Profile::new(
                    domain::Username::new(row.author_username).expect("invalid username"),
                    if row.author_bio.is_empty() { None } else { Some(row.author_bio) },
                    if row.author_image.is_empty() { None } else { Some(domain::ImageUrl::new(row.author_image).expect("invalid image")) },
                    row.following_author,
                ),
            }
        }).collect();

        Ok(ArticlesEnvelope {
            articles,
            articles_count: count as usize,
        })
    }

    async fn favorite_article(&self, user_id: UserId, article_id: ArticleId) -> anyhow::Result<()> {
        let client = self.pool.get().await?;
        crate::clorinde::queries::articles::favorite_article()
            .bind(&client, &user_id.into(), &article_id.into())
            .await?;
        Ok(())
    }

    async fn unfavorite_article(
        &self,
        user_id: UserId,
        article_id: ArticleId,
    ) -> anyhow::Result<()> {
        let client = self.pool.get().await?;
        crate::clorinde::queries::articles::unfavorite_article()
            .bind(&client, &user_id.into(), &article_id.into())
            .await?;
        Ok(())
    }

    async fn is_favorited(&self, user_id: UserId, article_id: ArticleId) -> anyhow::Result<bool> {
        let client = self.pool.get().await?;
        let is_favorited = crate::clorinde::queries::articles::is_favorited()
            .bind(&client, &user_id.into(), &article_id.into())
            .one()
            .await?;
        Ok(is_favorited)
    }
}

#[derive(Clone)]
pub struct PostgresCommentsRepository {
    pool: Pool,
}

impl PostgresCommentsRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommentsRepository for PostgresCommentsRepository {
    async fn create_comment(&self, comment: Comment) -> anyhow::Result<Comment> {
        let client = self.pool.get().await?;
        let created = crate::clorinde::queries::comments::create_comment()
            .bind(
                &client,
                &comment.body,
                &comment.article_id.into(),
                &comment.author_id.into(),
                &comment.created_at.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()),
            )
            .one()
            .await?;
        
        Ok(Comment {
            id: CommentId::from(created.id as i64),
            body: created.body,
            article_id: ArticleId::from(created.article_id),
            author_id: UserId::from(created.author_id),
            created_at: created.created_at.with_timezone(&chrono::Utc),
            updated_at: created.updated_at.with_timezone(&chrono::Utc),
        })
    }

    async fn get_comments_by_article(&self, article_id: ArticleId) -> anyhow::Result<Vec<Comment>> {
        let client = self.pool.get().await?;
        let rows = crate::clorinde::queries::comments::get_comments_by_article()
            .bind(&client, &article_id.into())
            .all()
            .await?;
            
        Ok(rows.into_iter().map(|row| Comment {
            id: CommentId::from(row.id as i64),
            body: row.body,
            article_id: ArticleId::from(row.article_id),
            author_id: UserId::from(row.author_id),
            created_at: row.created_at.with_timezone(&chrono::Utc),
            updated_at: row.updated_at.with_timezone(&chrono::Utc),
        }).collect())
    }

    async fn delete_comment(&self, id: CommentId) -> anyhow::Result<()> {
        let client = self.pool.get().await?;
        crate::clorinde::queries::comments::delete_comment()
            .bind(&client, &(id.as_i64() as i32)) // Comment ID is int in DB
            .await?;
        Ok(())
    }

    async fn get_comment_by_id(&self, id: CommentId) -> anyhow::Result<Option<Comment>> {
        let client = self.pool.get().await?;
        let row = crate::clorinde::queries::comments::get_comment_by_id()
            .bind(&client, &(id.as_i64() as i32))
            .opt()
            .await?;
            
        Ok(row.map(|row| Comment {
            id: CommentId::from(row.id as i64),
            body: row.body,
            article_id: ArticleId::from(row.article_id),
            author_id: UserId::from(row.author_id),
            created_at: row.created_at.with_timezone(&chrono::Utc),
            updated_at: row.updated_at.with_timezone(&chrono::Utc),
        }))
    }
}

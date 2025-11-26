# Migration Progress Report

**Date**: 2025-11-24
**Status**: Partial Migration Complete

## ✅ Completed Migrations

### 1. Route Handler Files

| File | Status | Notes |
|------|--------|-------|
| `apps/api/src/auth.rs` | ✅ Complete | Uses `users_repo.get_user_by_id()` |
| `apps/api/src/routes/api/users.rs` | ✅ Complete | Register & login use repository |
| `apps/api/src/routes/api/current_user.rs` | ✅ Complete | Get/update user via repository |
| `apps/api/src/routes/api/profiles.rs` | ✅ Complete | Follow/unfollow use repository |
| `apps/api/src/routes/api/tags.rs` | ✅ Complete | Uses in-memory tags (acceptable) |
| `apps/api/src/routes/api/articles.rs` | ⚠️ Partial | `list_articles` & `feed_articles` migrated, others need work |

### 2. Changes Made

**auth.rs:**
```rust
// OLD
let users = state.users.read().await;
let user = users.iter().find(|u| u.id == user_id).cloned();

// NEW
let user = state.use_cases.users_repo.get_user_by_id(user_id).await?;
```

**users.rs:**
```rust
// Registration - OLD
let mut users = state.users.write().await;
if users.iter().any(|u| u.email == new_user.email) {
    return Err(ApiError::conflict("email already registered"));
}
users.push(new_user);

// Registration - NEW
if state.use_cases.users_repo.get_user_by_email(email.as_str()).await?.is_some() {
    return Err(ApiError::conflict("email already registered"));
}
let user = state.use_cases.users_repo.create_user(user).await?;
```

**profiles.rs:**
```rust
// OLD
let mut followers = state.followers.write().await;
add_follower(&mut followers, target.id, user.id);

// NEW
state.use_cases.users_repo.follow_user(user.id, target.id).await?;
```

**articles.rs (partial):**
```rust
// List articles - NEW
let envelope = state.use_cases.articles_repo.list_articles(filters).await?;
for article in envelope.articles {
    let author = state.use_cases.users_repo.get_user_by_id(article.author_id).await?;
    let following = state.use_cases.users_repo.is_following(viewer, article.author_id).await?;
    let favorited = state.use_cases.articles_repo.is_favorited(viewer, article.id).await?;
    // ... build summaries
}
```

## ⚠️ Remaining Work: articles.rs

The following functions in `articles.rs` still need migration:

### Functions to Migrate

1. **`update_article`** - Currently uses `state.articles.write().await`
2. **`delete_article`** - Currently uses `state.articles.write().await` 
3. **`favorite_article`** - Currently uses `state.favorites` and `state.articles`
4. **`unfavorite_article`** - Currently uses `state.favorites` and `state.articles`
5. **`create_comment`** - Uses `state.comments` and `state.next_comment_id`
6. **`list_comments`** - Uses `state.comments`
7. **`delete_comment`** - Uses `state.comments`
8. **Helper functions** - `article_identifiers`, `author_profile_with_viewer`, etc.

### Migration Patterns for articles.rs

#### Pattern 1: update_article
```rust
async fn update_article(
    State(state): State<AppState>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
    Json(req): Json<UpdateArticleRequest>,
) -> ApiResult<Json<ArticleEnvelope>> {
    // Get article
    let mut article = state.use_cases.articles_repo
        .get_article_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::not_found("article"))?;
    
    // Authorization check
    if article.author_id != user.id {
        return Err(ApiError::unauthorized("cannot edit another user's article"));
    }
    
    // Apply changes
    let mut changes = ArticleChanges {
        title: req.article.title,
        description: req.article.description,
        body: req.article.body,
        ..Default::default()
    };
    if let Some(tags) = req.article.tag_list {
        changes.tag_list = Some(TagList::new(tags)?);
    }
    article.apply_changes(changes, Utc::now())?;
    
    // Update in repository
    let updated = state.use_cases.articles_repo.update_article(article).await?;
    
    // Build response
    let favorited = state.use_cases.articles_repo
        .is_favorited(user.id, updated.id)
        .await
        .unwrap_or(false);
    let profile = user.to_profile(false);
    let view = updated.to_view(profile, favorited);
    Ok(Json(ArticleEnvelope::from(view)))
}
```

#### Pattern 2: delete_article
```rust
async fn delete_article(
    State(state): State<AppState>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
) -> ApiResult<()> {
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
```

#### Pattern 3: favorite/unfavorite_article
```rust
async fn favorite_article(
    State(state): State<AppState>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
) -> ApiResult<Json<ArticleEnvelope>> {
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
```

#### Pattern 4: Comments (Need CommentsRepository in UseCases)

First, add CommentsRepository to UseCases:

```rust
// In domain/src/use_cases/mod.rs
pub struct UseCases<U, A, C> 
where
    U: UsersRepository,
    A: ArticlesRepository,
    C: CommentsRepository,
{
    pub users_repo: U,
    pub articles_repo: A,
    pub comments_repo: C,
}
```

Then:
```rust
async fn create_comment(
    State(state): State<AppState>,
    CurrentUser { user, .. }: CurrentUser,
    Path(slug): Path<String>,
    Json(req): Json<CreateCommentRequest>,
) -> ApiResult<Json<CommentEnvelope>> {
    let article = state.use_cases.articles_repo
        .get_article_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::not_found("article"))?;
    
    let draft = CommentDraft::new(req.comment.body)?;
    let comment = Comment::create(
        CommentId::default(), // Let DB generate
        article.id,
        user.id,
        draft,
        Utc::now(),
    );
    
    let created = state.use_cases.comments_repo.create_comment(comment).await?;
    let profile = user.to_profile(false);
    let view = created.to_view(profile);
    Ok(Json(CommentEnvelope::from(view)))
}
```

## Build Status

Current compilation errors: ~25 (all in articles.rs)

```bash
# Check errors
cargo build -p api 2>&1 | grep "^error" | wc -l
```

## Testing with In-Memory Repositories

### Option 1: Conditional Compilation

Add test-only constructor in AppState:

```rust
#[cfg(test)]
impl AppState {
    pub fn with_in_memory_repos() -> Self {
        let users_repo = domain::repositories::InMemoryUsersRepository::new();
        let articles_repo = domain::repositories::InMemoryArticlesRepository::new();
        let use_cases = domain::use_cases::UseCases::new(users_repo, articles_repo);
        
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            use_cases: Arc::new(use_cases),
            tags: Arc::new(RwLock::new(TagList::default())),
        }
    }
}
```

**Problem**: Type mismatch - `UseCases` in production uses `PostgresXRepository`, tests need `InMemoryXRepository`.

### Option 2: Make AppState Generic (Recommended)

```rust
#[derive(Clone)]
pub struct AppState<U, A> 
where
    U: UsersRepository,
    A: ArticlesRepository,
{
    pub sessions: Arc<RwLock<HashMap<String, UserId>>>,
    pub use_cases: Arc<UseCases<U, A>>,
    pub tags: Arc<RwLock<TagList>>,
}

// Type alias for production
pub type ProductionAppState = AppState<
    data::PostgresUsersRepository,
    data::PostgresArticlesRepository
>;

// Type alias for tests
#[cfg(test)]
pub type TestAppState = AppState<
    domain::repositories::InMemoryUsersRepository,
    domain::repositories::InMemoryArticlesRepository
>;
```

Then update route signatures:
```rust
pub fn router<U, A>() -> Router<AppState<U, A>>
where
    U: UsersRepository + Clone,
    A: ArticlesRepository + Clone,
{
    // ...
}
```

### Option 3: Trait Objects (Simplest for now)

```rust
pub struct AppState {
    pub sessions: Arc<RwLock<HashMap<String, UserId>>>,
    pub users_repo: Arc<dyn UsersRepository>,
    pub articles_repo: Arc<dyn ArticlesRepository>,
    pub tags: Arc<RwLock<TagList>>,
}
```

**Trade-off**: Small runtime overhead, but simpler code.

## Next Steps

1. **Complete articles.rs migration** - Apply patterns above to remaining functions
2. **Add CommentsRepository to UseCases** - Update UseCases struct to include comments
3. **Decide on testing strategy** - Choose between Option 2 (generic) or Option 3 (trait objects)
4. **Update tests** - Once compilation passes, update tests to use in-memory repos
5. **Run tests** - `cargo test -p api`
6. **Integration tests** - Add database integration tests with testcontainers

## Estimated Time to Complete

- Finish articles.rs migration: 30-60 minutes
- Add CommentsRepository: 15 minutes
- Implement testing strategy: 30-60 minutes
- Fix tests: 30-60 minutes

**Total**: 2-3 hours of focused work

## Backup Files

- `apps/api/src/routes/api/articles.rs.bak` - Original articles.rs
- `apps/api/src/state/mod.rs.bak` - Original AppState with in-memory state

## Commands

```bash
# See remaining errors
cargo build -p api 2>&1 | grep "^error"

# Test specific file after migration
cargo test -p api --lib routes::api::users

# Run all tests
cargo test -p api

# Check clippy
cargo clippy -p api -- -D warnings
```

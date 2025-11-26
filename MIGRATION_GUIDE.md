# Migration Guide: From In-Memory State to Repository Pattern

## Overview

The `AppState` has been simplified to only hold:
- `sessions`: Temporary session storage (to be replaced with JWT)
- `use_cases`: Contains repository implementations for database access

## Old vs New AppState

### Before:
```rust
pub struct AppState {
    pub tags: Arc<RwLock<TagList>>,
    pub users: Arc<RwLock<Vec<User>>>,
    pub sessions: Arc<RwLock<HashMap<String, UserId>>>,
    pub articles: Arc<RwLock<Vec<Article>>>,
    pub favorites: Arc<RwLock<HashMap<ArticleId, HashSet<UserId>>>>,
    pub comments: Arc<RwLock<Vec<Comment>>>,
    pub next_comment_id: Arc<RwLock<i64>>,
    pub followers: Arc<RwLock<HashMap<UserId, HashSet<UserId>>>>,
}
```

### After:
```rust
pub struct AppState {
    pub sessions: Arc<RwLock<HashMap<String, UserId>>>,
    pub use_cases: Arc<UseCases<PostgresUsersRepository, PostgresArticlesRepository>>,
}
```

## Migration Patterns

### Pattern 1: Reading Users

#### Before:
```rust
let users = state.users.read().await;
let user = users.iter()
    .find(|u| u.id == user_id)
    .cloned()
    .ok_or(ApiError::not_found("user"))?;
```

#### After:
```rust
let user = state.use_cases.users_repo
    .get_user_by_id(user_id)
    .await
    .map_err(|_| ApiError::internal_error())?
    .ok_or_else(|| ApiError::not_found("user"))?;
```

### Pattern 2: Creating Users

#### Before:
```rust
let mut users = state.users.write().await;
if users.iter().any(|u| u.email.as_str() == new_user.email.as_str()) {
    return Err(ApiError::conflict("email already registered"));
}
users.push(new_user.clone());
```

#### After:
```rust
// Check if exists
if state.use_cases.users_repo
    .get_user_by_email(new_user.email.as_str())
    .await
    .map_err(|_| ApiError::internal_error())?
    .is_some() 
{
    return Err(ApiError::conflict("email already registered"));
}

let user = state.use_cases.users_repo
    .create_user(new_user)
    .await
    .map_err(|_| ApiError::internal_error())?;
```

### Pattern 3: Listing Articles

#### Before:
```rust
let result = state.list_articles(filters).await;
let users = state.users.read().await.clone();
let favorites = state.favorites.read().await.clone();
// ... complex filtering logic
```

#### After:
```rust
let articles_envelope = state.use_cases.articles_repo
    .list_articles(filters)
    .await
    .map_err(|_| ApiError::internal_error())?;
```

**Note**: The repository's `list_articles` method should return `ArticlesEnvelope` which contains both articles and count. The filtering/pagination logic moves to the repository.

### Pattern 4: Following/Unfollowing Users

#### Before:
```rust
let mut followers = state.followers.write().await;
followers.entry(followee_id)
    .or_insert_with(HashSet::new)
    .insert(follower_id);
```

#### After:
```rust
state.use_cases.users_repo
    .follow_user(follower_id, followee_id)
    .await
    .map_err(|_| ApiError::internal_error())?;
```

### Pattern 5: Favoriting Articles

#### Before:
```rust
let mut favorites = state.favorites.write().await;
favorites.entry(article_id)
    .or_insert_with(HashSet::new)
    .insert(user_id);

let mut articles = state.articles.write().await;
if let Some(article) = articles.iter_mut().find(|a| a.id == article_id) {
    article.favorites_count += 1;
}
```

#### After:
```rust
state.use_cases.articles_repo
    .favorite_article(user_id, article_id)
    .await
    .map_err(|_| ApiError::internal_error())?;
```

### Pattern 6: Creating Comments

#### Before:
```rust
let mut next_id = state.next_comment_id.write().await;
let id = CommentId::from(*next_id);
*next_id += 1;

let comment = Comment { id, article_id, author_id, body, created_at, updated_at };
state.comments.write().await.push(comment.clone());
```

#### After:
```rust
let comment = Comment {
    id: CommentId::default(), // or let DB generate
    article_id,
    author_id,
    body,
    created_at,
    updated_at,
};

let created = state.use_cases.comments_repo
    .create_comment(comment)
    .await
    .map_err(|_| ApiError::internal_error())?;
```

## Step-by-Step Migration Checklist

### Files to Update:

- [x] `apps/api/src/state/mod.rs` - Simplified AppState
- [x] `apps/api/src/main.rs` - Wire repositories
- [ ] `apps/api/src/auth.rs` - Use repository for user lookup
- [ ] `apps/api/src/routes/api/users.rs` - Register/Login
- [ ] `apps/api/src/routes/api/current_user.rs` - Get/Update current user
- [ ] `apps/api/src/routes/api/profiles.rs` - Follow/Unfollow
- [ ] `apps/api/src/routes/api/articles.rs` - CRUD + Favorite + Comments
- [ ] `apps/api/src/routes/api/tags.rs` - Get tags (may need Tags table)

### Current Status:

✅ Domain layer - Repository traits defined
✅ Data layer - Postgres implementations created  
✅ API layer - Wiring complete
⚠️ Route handlers - Need migration (41 compilation errors)

## Migration Order (Recommended)

1. **auth.rs** - User lookup (DONE)
2. **users.rs** - Registration and login
3. **current_user.rs** - Get/Update user
4. **profiles.rs** - Follow/Unfollow operations
5. **articles.rs** - Complex: articles CRUD, favorites, comments
6. **tags.rs** - May need a tags table or keep in-memory

## Testing Strategy

After each file migration:
1. Build: `cargo build -p api`
2. Run: `cargo run -p api`
3. Test endpoints with curl or Postman
4. Fix any runtime errors

## Helper Methods Needed

You may want to add these to your repositories:

### UsersRepository
- ✅ `get_user_by_id`
- ✅ `get_user_by_email`
- ✅ `get_user_by_username`
- ✅ `create_user`
- ✅ `update_user`
- ✅ `follow_user`
- ✅ `unfollow_user`
- ✅ `is_following`

### ArticlesRepository
- ✅ `create_article`
- ✅ `get_article_by_slug`
- ✅ `get_article_by_id`
- ✅ `update_article`
- ✅ `delete_article`
- ✅ `list_articles` (with filters)
- ✅ `feed_articles`
- ✅ `favorite_article`
- ✅ `unfavorite_article`
- ✅ `is_favorited`

### CommentsRepository
- ✅ `create_comment`
- ✅ `get_comments_by_article`
- ✅ `delete_comment`
- ✅ `get_comment_by_id`

## Tags Consideration

Tags were previously in-memory. Options:
1. **Keep in-memory**: Tags rarely change, cache them at startup
2. **Add to DB**: Create a `tags` table with unique constraint
3. **Extract from articles**: Query distinct tags from articles table

Recommendation: Option 2 or 3 for consistency.

## Sessions Consideration

Currently using in-memory `HashMap<String, UserId>`. This will not work in production with multiple instances.

Options:
1. **JWT tokens**: Stateless, no session storage needed
2. **Redis**: Distributed session store
3. **Database**: Store sessions in Postgres

Recommendation: Implement JWT authentication (most common for APIs).

## Next Steps

1. Fix remaining route handlers one by one following the patterns above
2. Test each endpoint after migration
3. Remove old test code from `state/mod.rs.bak` once confident
4. Implement proper authentication (JWT)
5. Add integration tests for database operations

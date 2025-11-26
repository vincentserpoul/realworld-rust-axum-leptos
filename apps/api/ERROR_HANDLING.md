# Error Handling with AppError and RFC9457

This API implements error handling using a custom `AppError` type that follows the pattern described in [Learning Rust: Custom Errors](https://rup12.net/posts/learning-rust-custom-errors/) and returns RFC9457-compliant Problem Details responses.

## Overview

The `AppError` type is a newtype wrapper around `anyhow::Error` that:
- Allows using the `?` operator throughout handler functions
- Automatically converts various error types into appropriate HTTP responses
- Returns RFC9457 Problem Details JSON format
- Provides proper HTTP status codes based on error types

## Implementation

```rust
#[derive(Debug)]
pub struct AppError(anyhow::Error);

pub type AppResult<T> = Result<T, AppError>;
```

## RFC9457 Problem Details Format

All errors are returned as JSON with `Content-Type: application/problem+json`:

```json
{
  "type": "about:blank#404",
  "title": "Not Found",
  "status": 404,
  "detail": "User not found"
}
```

### Fields

- `type`: A URI reference identifying the problem type (defaults to about:blank with status code)
- `title`: A short, human-readable summary of the problem type
- `status`: The HTTP status code
- `detail`: A human-readable explanation specific to this occurrence of the problem
- `instance`: (optional) A URI reference identifying the specific occurrence

## Usage Examples

### Using the `?` operator

The beauty of this approach is that you can use `?` to propagate errors:

```rust
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<UserId>,
) -> AppResult<Json<UserResponse>> {
    let user = User::find_by_id(&state.db, id).await?; // DomainError -> AppError
    let data = process_user(user)?; // anyhow::Error -> AppError
    Ok(Json(data))
}
```

### Explicit error construction

You can also create errors explicitly:

```rust
if email_exists {
    return Err(AppError::conflict("email already registered"));
}

if !authorized {
    return Err(AppError::unauthorized("invalid credentials"));
}

if user.is_none() {
    return Err(AppError::not_found("user not found"));
}
```

## Error Mapping

### From DomainError

Domain errors are automatically mapped to appropriate HTTP status codes:

- `DomainError::Conflict` → 409 Conflict
- `DomainError::NotFound` → 404 Not Found
- `DomainError::UnauthorizedAction` → 401 Unauthorized
- Other domain errors → 422 Unprocessable Entity

### From anyhow::Error

Generic `anyhow::Error` instances are mapped based on their message content:

- Contains "already registered" or "already exists" → 409 Conflict
- Contains "not found" → 404 Not Found
- Contains "invalid credentials" or "unauthorized" → 401 Unauthorized
- Contains "cannot follow" or "validation" → 422 Unprocessable Entity
- Default → 500 Internal Server Error

## Example Responses

### 404 Not Found

```http
HTTP/1.1 404 Not Found
Content-Type: application/problem+json

{
  "type": "about:blank#404",
  "title": "Not Found",
  "status": 404,
  "detail": "User not found"
}
```

### 401 Unauthorized

```http
HTTP/1.1 401 Unauthorized
Content-Type: application/problem+json

{
  "type": "about:blank#401",
  "title": "Unauthorized",
  "status": 401,
  "detail": "invalid credentials"
}
```

### 409 Conflict

```http
HTTP/1.1 409 Conflict
Content-Type: application/problem+json

{
  "type": "about:blank#409",
  "title": "Conflict",
  "status": 409,
  "detail": "email already registered"
}
```

### 422 Unprocessable Entity

```http
HTTP/1.1 422 Unprocessable Entity
Content-Type: application/problem+json

{
  "type": "about:blank#422",
  "title": "Validation Error",
  "status": 422,
  "detail": "cannot follow yourself"
}
```

## Benefits

1. **Clean Handler Code**: No manual error handling cluttering business logic
2. **Consistent Responses**: All errors follow RFC9457 standard
3. **Type Safety**: Rust's type system ensures all errors are handled
4. **Flexibility**: Easy to add new error types via `From` implementations
5. **Standards Compliant**: RFC9457 is widely supported by HTTP clients and tooling

## Adding New Error Types

To support conversion from new error types, simply implement `From`:

```rust
impl From<MyCustomError> for AppError {
    fn from(err: MyCustomError) -> Self {
        Self(err.into())
    }
}
```

Since `AppError` wraps `anyhow::Error`, any type that can be converted to `anyhow::Error` can be converted to `AppError`.

## References

- [RFC9457 - Problem Details for HTTP APIs](https://www.rfc-editor.org/rfc/rfc9457.html)
- [Learning Rust: Custom Errors](https://rup12.net/posts/learning-rust-custom-errors/)
- [anyhow documentation](https://docs.rs/anyhow/)

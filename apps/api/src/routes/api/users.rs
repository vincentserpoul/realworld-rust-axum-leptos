use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use chrono::Utc;
use domain::{AuthToken, Email, PasswordHash, PlainPassword, User, UserEnvelope, UserId, Username};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/", post(register_user))
        .route("/login", post(login_user))
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    user: RegisterPayload,
}

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    user: LoginPayload,
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    email: String,
    password: String,
}

async fn register_user(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> ApiResult<impl IntoResponse> {
    let RegisterPayload {
        username,
        email,
        password,
    } = req.user;

    let username = Username::new(username)?;
    let email = Email::parse(email)?;
    let password = PlainPassword::new(password)?;

    let password_hash = hash_password(&password)?;
    let now = Utc::now();
    let user = User::new(UserId::random(), email, username, password_hash, now);

    let mut users = state.users.write().await;
    if users
        .iter()
        .any(|existing| existing.email.as_str() == user.email.as_str())
    {
        return Err(ApiError::conflict("email already registered"));
    }
    users.push(user.clone());
    drop(users);

    let token = issue_token()?;
    remember_session(&state, &token, user.id).await;
    let view = user.to_view(Some(token));

    Ok((StatusCode::CREATED, Json(UserEnvelope::from(view))))
}

async fn login_user(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> ApiResult<impl IntoResponse> {
    let LoginPayload { email, password } = req.user;
    let email = Email::parse(email)?;
    let password = PlainPassword::new(password)?;
    let password_hash = hash_password(&password)?;

    let users = state.users.read().await;
    let Some(user) = users
        .iter()
        .find(|candidate| candidate.email.as_str() == email.as_str())
    else {
        return Err(ApiError::unauthorized("invalid credentials"));
    };

    if user.password_hash != password_hash {
        return Err(ApiError::unauthorized("invalid credentials"));
    }

    let user = user.clone();
    drop(users);

    let token = issue_token()?;
    remember_session(&state, &token, user.id).await;
    let view = user.to_view(Some(token));
    Ok(Json(UserEnvelope::from(view)))
}

pub(crate) fn hash_password(password: &PlainPassword) -> ApiResult<PasswordHash> {
    let salted = format!("hashed:{}", password.as_str());
    PasswordHash::new(salted).map_err(ApiError::from)
}

fn issue_token() -> ApiResult<AuthToken> {
    AuthToken::new(Uuid::new_v4().to_string()).map_err(ApiError::from)
}

async fn remember_session(state: &AppState, token: &AuthToken, user_id: UserId) {
    state
        .sessions
        .write()
        .await
        .insert(token.as_str().to_owned(), user_id);
}

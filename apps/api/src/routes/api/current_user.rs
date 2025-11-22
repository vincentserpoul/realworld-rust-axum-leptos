use axum::{
    extract::State,
    routing::get,
    Json, Router,
};
use chrono::Utc;
use domain::{Email, ImageUrl, PlainPassword, UpdateUserInput, UserEnvelope, Username};
use serde::Deserialize;

use crate::{
    auth::CurrentUser,
    error::{ApiError, ApiResult},
    state::AppState,
};

use super::users::hash_password;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/user", get(get_current_user).put(update_current_user))
}

async fn get_current_user(
    CurrentUser { user, token }: CurrentUser,
) -> ApiResult<Json<UserEnvelope>> {
    let view = user.to_view(Some(token));
    Ok(Json(UserEnvelope::from(view)))
}

#[derive(Debug, Deserialize, Default)]
struct UpdateUserRequest {
    user: UpdateUserPayload,
}

#[derive(Debug, Deserialize, Default)]
struct UpdateUserPayload {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
    bio: Option<String>,
    image: Option<String>,
}

async fn update_current_user(
    State(state): State<AppState>,
    CurrentUser { user, token }: CurrentUser,
    Json(req): Json<UpdateUserRequest>,
) -> ApiResult<Json<UserEnvelope>> {
    let UpdateUserPayload {
        email,
        username,
        password,
        bio,
        image,
    } = req.user;

    let mut users = state.users.write().await;
    let current_id = user.id;

    let parsed_email = if let Some(email) = email {
        let email = Email::parse(email)?;
        if users.iter().any(|candidate| {
            candidate.id != current_id && candidate.email.as_str() == email.as_str()
        }) {
            return Err(ApiError::conflict("email already registered"));
        }
        Some(email)
    } else {
        None
    };

    let Some(existing) = users
        .iter_mut()
        .find(|candidate| candidate.id == current_id)
    else {
        return Err(ApiError::not_found("user"));
    };

    let mut changes = UpdateUserInput::default();

    if let Some(email) = parsed_email {
        changes.email = Some(email);
    }

    if let Some(username) = username {
        changes.username = Some(Username::new(username)?);
    }

    if let Some(bio) = bio {
        changes.bio = Some(Some(bio));
    }

    if let Some(image) = image {
        changes.image = Some(Some(ImageUrl::new(image)?));
    }

    if let Some(password) = password {
        let password = PlainPassword::new(password)?;
        let password_hash = hash_password(&password)?;
        changes.password_hash = Some(password_hash);
    }

    existing.apply_update(changes, Utc::now());
    let view = existing.to_view(Some(token));
    Ok(Json(UserEnvelope::from(view)))
}

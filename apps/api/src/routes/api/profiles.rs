use std::collections::HashSet;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
    routing::post,
};
use domain::{Profile, ProfileEnvelope, User, UserId};

use crate::{
    auth::CurrentUser,
    error::{ApiError, ApiResult},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/:username", get(get_profile)).route(
        "/:username/follow",
        post(follow_profile).delete(unfollow_profile),
    )
}

async fn get_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
    current_user: Option<CurrentUser>,
) -> ApiResult<Json<ProfileEnvelope>> {
    let viewer_id = current_user.as_ref().map(|current| current.user.id);
    let user = find_user_by_username(&state, &username).await?;
    let profile = to_profile(&state, user, viewer_id).await?;
    Ok(Json(ProfileEnvelope::from(profile)))
}

async fn follow_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
    CurrentUser { user, .. }: CurrentUser,
) -> ApiResult<Json<ProfileEnvelope>> {
    let target = find_user_by_username(&state, &username).await?;
    if target.id == user.id {
        return Err(ApiError::validation("cannot follow yourself"));
    }

    {
        let mut followers = state.followers.write().await;
        let entry = followers.entry(target.id).or_insert_with(HashSet::new);
        entry.insert(user.id);
    }

    let profile = to_profile(&state, target, Some(user.id)).await?;
    Ok(Json(ProfileEnvelope::from(profile)))
}

async fn unfollow_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
    CurrentUser { user, .. }: CurrentUser,
) -> ApiResult<Json<ProfileEnvelope>> {
    let target = find_user_by_username(&state, &username).await?;
    if target.id == user.id {
        return Err(ApiError::validation("cannot unfollow yourself"));
    }

    {
        let mut followers = state.followers.write().await;
        if let Some(entry) = followers.get_mut(&target.id) {
            entry.remove(&user.id);
        }
    }

    let profile = to_profile(&state, target, Some(user.id)).await?;
    Ok(Json(ProfileEnvelope::from(profile)))
}

async fn find_user_by_username(state: &AppState, username: &str) -> ApiResult<User> {
    let users = state.users.read().await;
    users
        .iter()
        .find(|candidate| candidate.username.as_str() == username)
        .cloned()
        .ok_or_else(|| ApiError::not_found("profile"))
}

async fn to_profile(state: &AppState, user: User, viewer_id: Option<UserId>) -> ApiResult<Profile> {
    let following = match viewer_id {
        Some(viewer) => is_following(state, user.id, viewer).await,
        None => false,
    };
    Ok(user.to_profile(following))
}

async fn is_following(state: &AppState, target_id: UserId, follower_id: UserId) -> bool {
    state
        .followers
        .read()
        .await
        .get(&target_id)
        .map(|followers| followers.contains(&follower_id))
        .unwrap_or(false)
}

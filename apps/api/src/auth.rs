use axum::{
    extract::{FromRequestParts, OptionalFromRequest, Request},
    http::{header::AUTHORIZATION, request::Parts},
};
use domain::{AuthToken, User};

use crate::{error::ApiError, state::AppState};

#[derive(Clone)]
pub struct CurrentUser {
    pub user: User,
    pub token: AuthToken,
}

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = ApiError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let token_header = parts.headers.get(AUTHORIZATION).cloned();
        let state = state.clone();
        async move {
            let header = token_header
                .ok_or_else(|| ApiError::unauthorized("missing authorization header"))?;
            let header = header
                .to_str()
                .map_err(|_| ApiError::unauthorized("invalid authorization header"))?;
            let token_value = header
                .strip_prefix("Token ")
                .or_else(|| header.strip_prefix("Bearer "))
                .unwrap_or(header)
                .trim();
            if token_value.is_empty() {
                return Err(ApiError::unauthorized("invalid authorization header"));
            }

            let sessions = state.sessions.read().await;
            let user_id = sessions
                .get(token_value)
                .copied()
                .ok_or_else(|| ApiError::unauthorized("invalid token"))?;
            drop(sessions);

            let users = state.users.read().await;
            let user = users
                .iter()
                .find(|candidate| candidate.id == user_id)
                .cloned()
                .ok_or_else(|| ApiError::not_found("user"))?;

            let token = AuthToken::new(token_value.to_owned()).map_err(ApiError::from)?;
            Ok(Self { user, token })
        }
    }
}

// Use blanket FromRequest implementation provided by axum_core via FromRequestParts.

impl OptionalFromRequest<AppState> for CurrentUser {
    type Rejection = ApiError;

    fn from_request(
        request: Request,
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Option<Self>, Self::Rejection>> + Send {
        let state = state.clone();
        async move {
            let (mut parts, _) = request.into_parts();
            match <Self as FromRequestParts<AppState>>::from_request_parts(&mut parts, &state)
                .await
            {
                Ok(user) => Ok(Some(user)),
                Err(_) => Ok(None),
            }
        }
    }
}



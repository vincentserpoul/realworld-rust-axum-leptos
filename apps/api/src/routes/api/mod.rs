mod articles;
mod current_user;
mod profiles;
mod tags;
mod users;

use crate::state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .nest("/articles", articles::router())
        .nest("/tags", tags::router())
        .nest("/profiles", profiles::router())
        .nest("/users", users::router())
        .merge(current_user::router())
}

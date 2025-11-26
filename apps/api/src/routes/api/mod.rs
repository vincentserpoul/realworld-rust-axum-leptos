mod articles;
mod current_user;
mod profiles;
mod tags;
mod users;

use crate::state::AppState;
use axum::Router;

pub fn router<U, A, C>() -> Router<AppState<U, A, C>>
where
    U: domain::repositories::UsersRepository + Clone + 'static,
    A: domain::repositories::ArticlesRepository + Clone + 'static,
    C: domain::repositories::CommentsRepository + Clone + 'static,
{
    Router::<AppState<U, A, C>>::new()
        .nest("/articles", articles::router())
        .nest("/tags", tags::router())
        .nest("/profiles", profiles::router())
        .nest("/users", users::router())
        .merge(current_user::router())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_exists() {
        // This test exists to ensure the module structure is valid
        // The router() function is tested implicitly through integration tests
    }
}

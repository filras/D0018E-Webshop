use axum::Router;

mod users;

pub fn router() -> Router {
    Router::new()
        .merge(users::routes())
}

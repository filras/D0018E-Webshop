use axum::Router;

mod users;
mod items;

pub fn router() -> Router {
    Router::new()
        .merge(users::routes())
        .merge(items::routes())
}

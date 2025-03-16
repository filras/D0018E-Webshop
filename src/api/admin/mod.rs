use axum::Router;

mod users;
mod items;
mod orders;

pub fn router() -> Router {
    Router::new()
        .merge(users::routes())
        .merge(items::routes())
        .merge(orders::routes())
}

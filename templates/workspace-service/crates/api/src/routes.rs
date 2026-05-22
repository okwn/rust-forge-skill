use axum::{routing::{get, post, delete}, Router};
use super::handlers::*;

pub fn routes() -> Router {
    Router::new()
        .route("/items", get(list_items).post(create_item))
        .route("/items/:id", get(get_item).delete(delete_item))
}
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{handlers, state::SharedState};

pub fn build_router(state: SharedState) -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route(
            "/items",
            get(handlers::list_items).post(handlers::create_item),
        )
        .route(
            "/items/:id",
            get(handlers::get_item)
                .put(handlers::update_item)
                .delete(handlers::delete_item),
        )
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

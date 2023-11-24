use crate::handlers::user_handlers;
use axum::routing::{delete, get, put};
use axum::{http::Method, Router};
use tower_http::cors::{Any, CorsLayer};

pub fn user_routes() -> Router {
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let router = Router::new()
        .route(
            "/api/user/:uuid/update",
            put(user_handlers::update_user_put),
        )
        .route(
            "/api/user/:uuid/delete",
            delete(user_handlers::delete_user_delete),
        )
        .route("/api/user/all", get(user_handlers::all_user_get))
        .layer(cors);
    router
}

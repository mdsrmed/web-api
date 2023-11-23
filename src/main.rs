// use std::path::Path;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, Database, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};

mod handlers;
mod models;
mod routes;
mod utils;

#[tokio::main]

async fn main() {
    println!("Hello, world!");
    server().await;
}

async fn server() {
    let conn_str = (*utils::components::DATABASE_URL).clone();
    let db = Database::connect(conn_str)
        .await
        .expect("Failed to connect");
    // let db: DatabaseConnection =
    //     Database::connect("postgres://postgres:password@localhost:5432/webapi")
    //         .await
    //         .unwrap();

    let app: Router = Router::new()
        .merge(routes::auth_routes::auth_routes())
        .merge(routes::user_routes::user_routes())
        .layer(axum::Extension(db));
    // .route("/api/test", get(test))
    // .route("/api/user/register", post(create_user_post))
    // .route("/api/user/login", post(login_user_post))
    // .route("/api/user/:uuid/update", put(update_user_put))
    // .route("/api/user/:uuid/delete", delete(delete_user_delete))
    // .route("/api/user/all", get(all_user_get));

    axum::Server::bind(&"0.0.0.0:4040".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// async fn test() -> impl IntoResponse {
//     println!("test!");
//     (StatusCode::ACCEPTED, "hello")
// }

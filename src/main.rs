// use std::path::Path;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Utc;
use entity::user::{self, Model};
use models::user_models::{CreateUserModel, LoginUserModel, UpdateUserModel, UserModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, Database, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};
use uuid::Uuid;

mod handlers;
mod models;
mod routes;

#[tokio::main]

async fn main() {
    println!("Hello, world!");
    server().await;
}

async fn server() {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:password@localhost:5432/webapi")
            .await
            .unwrap();

    let app: Router = Router::new()
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



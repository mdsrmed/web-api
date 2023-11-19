use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use entity::user;
use models::user_models::CreateUserModel;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, Set};
use uuid::Uuid;

mod models;

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
        .route("/api/test", get(test))
        .route("/api/test/insert", post(create_user_post));

    axum::Server::bind(&"0.0.0.0:4040".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test() -> impl IntoResponse {
    println!("test!");
    (StatusCode::ACCEPTED, "hello")
}

async fn create_user_post(Json(user_data): Json<CreateUserModel>) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:password@localhost:5432/webapi")
            .await
            .unwrap();

    let user_model = user::ActiveModel {
        name: Set(user_data.name.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(user_data.password.to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    user_model.insert(&db).await.unwrap();

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, "Inserted")
}

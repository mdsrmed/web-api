use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use sea_orm::{Database, DatabaseConnection};

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

    let app: Router = Router::new().route("/api/test", get(test));

    axum::Server::bind(&"0.0.0.0:4040".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test() -> impl IntoResponse {
    println!("test!");
    (StatusCode::ACCEPTED, "hello")
}

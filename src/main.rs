use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[tokio::main]

async fn main() {
    println!("Hello, world!");
    server().await;
}

async fn server() {
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

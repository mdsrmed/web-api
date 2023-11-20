use std::path::Path;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Utc;
use entity::user;
use models::user_models::{CreateUserModel, LoginUserModel, UpdateUserModel, UserModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, Database, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};
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
        .route("/api/test/insert", post(create_user_post))
        .route("/api/user/login", post(login_user_post))
        .route("/api/user/:uuid/update", put(update_user_put))
        .route("/api/user/:uuid/delete", delete(delete_user_delete))
        .route("/api/user/all", get(all_user_get));

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

async fn login_user_post(Json(user_data): Json<LoginUserModel>) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:password@localhost:5432/webapi")
            .await
            .unwrap();

    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(user_data.email))
                .add(entity::user::Column::Password.eq(user_data.password)),
        )
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    let data = UserModel {
        name: user.name,
        email: user.email,
        password: user.password,
        uuid: user.uuid,
        created_at: user.created_at,
    };

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, Json(data))
}

async fn update_user_put(
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>,
) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:password@localhost:5432/webapi")
            .await
            .unwrap();

    let mut user: entity::user::ActiveModel = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    user.name = Set(user_data.name);
    user.updata(&db).await.unwrap();

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, "Updated")
}

async fn delete_user_delete(Path(uuid): Path<Uuid>) -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:password@localhost:5432/webapi")
            .await
            .unwrap();

    let mut user: entity::user::ActiveModel = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    entity::user::Entity::delete_by_id(user.id)
        .exec(&db)
        .await
        .unwrap();

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, "Deleted")
}

async fn all_user_get() -> impl IntoResponse {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:password@localhost:5432/webapi")
            .await
            .unwrap();

    let users: Vec<UserModel> = entity::user::Entity::find()
        .all(&db)
        .await
        .unwrap()
        .into_iter()
        .map(|item| UserModel {
            name: item.name,
            email: item.email,
            uuid: item.uuid,
            created_at: item.created_at,
            password: todo!(),
        })
        .collect();

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, Json(users))
}

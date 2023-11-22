use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use entity::user;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, Database, DatabaseConnection, EntityTrait, Set,
};
use uuid::Uuid;

use crate::models::user_models::{CreateUserModel, LoginUserModel, UserModel};

pub async fn create_user_post(Json(user_data): axum::Json<CreateUserModel>) -> impl IntoResponse {
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

pub async fn login_user_post(Json(user_data): Json<LoginUserModel>) -> impl IntoResponse {
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

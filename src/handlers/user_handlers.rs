use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use entity::user::Model;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::models::user_models::{UpdateUserModel, UserModel};

pub async fn update_user_put(
    axum::Extension(db): axum::Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    axum::Json(user_data): Json<UpdateUserModel>,
) -> impl IntoResponse {
    // let db: DatabaseConnection =
    //     Database::connect("postgres://postgres:password@localhost:5432/webapi")
    //         .await
    //         .unwrap();

    let mut user: entity::user::ActiveModel = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    user.name = Set(user_data.name);
    user.update(&db).await.unwrap();

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, "Updated")
}

pub async fn delete_user_delete(
    axum::Extension(db): axum::Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    // let db: DatabaseConnection =
    //     Database::connect("postgres://postgres:password@localhost:5432/webapi")
    //         .await
    //         .unwrap();

    let user: Model = entity::user::Entity::find()
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

pub async fn all_user_get() -> impl IntoResponse {
    // let db: DatabaseConnection =
    //     Database::connect("postgres://postgres:password@localhost:5432/webapi")
    //         .await
    //         .unwrap();

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

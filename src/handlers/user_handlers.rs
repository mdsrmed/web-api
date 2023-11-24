use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use entity::user::Model;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    models::user_models::{UpdateUserModel, UserModel},
    utils::api_errors::APIError,
};

pub async fn update_user_put(
    axum::Extension(db): axum::Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    axum::Json(user_data): Json<UpdateUserModel>,
) -> Result<(), APIError> {
    // let db: DatabaseConnection =
    //     Database::connect("postgres://postgres:password@localhost:5432/webapi")
    //         .await
    //         .unwrap();

    let mut user: entity::user::ActiveModel = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "Not Found".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        })?
        .into();

    user.name = Set(user_data.name);
    user.update(&db).await.map_err(|err| APIError {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(50),
    })?;

    // db.close().await.unwrap();
    // (StatusCode::ACCEPTED, "Updated")
    Ok(())
}

pub async fn delete_user_delete(
    axum::Extension(db): axum::Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> Result<(), APIError> {
    // let db: DatabaseConnection =
    //     Database::connect("postgres://postgres:password@localhost:5432/webapi")
    //         .await
    //         .unwrap();

    let user: Model = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "Not Found".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        })?
        .into();

    entity::user::Entity::delete_by_id(user.id)
        .exec(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    Ok(())
}

pub async fn all_user_get(
    db: axum::Extension<DatabaseConnection>,
) -> Result<Json<Vec<UserModel>>, APIError> {
    // let db: DatabaseConnection =
    //     Database::connect("postgres://postgres:password@localhost:5432/webapi")
    //         .await
    //         .unwrap();

    let users: Vec<UserModel> = entity::user::Entity::find()
        .all(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .into_iter()
        .map(|item| UserModel {
            name: item.name,
            email: item.email,
            uuid: item.uuid,
            created_at: item.created_at,
            password: todo!(),
        })
        .collect();

    Ok(Json(users))
}

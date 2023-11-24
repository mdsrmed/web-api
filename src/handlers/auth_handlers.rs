use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use entity::user;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::{
    models::user_models::{CreateUserModel, LoginUserModel, UserModel},
    utils::api_errors::APIError,
};

pub async fn create_user_post(
    axum::Extension(db): axum::Extension<DatabaseConnection>,
    Json(user_data): axum::Json<CreateUserModel>,
) -> Result<(), APIError> {
    // let db: DatabaseConnection =
    //     Database::connect("postgres://postgres:password@localhost:5432/webapi")
    //         .await
    //         .unwrap();

    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(user_data.email.clone()))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    if user != None {
        return Err(APIError {
            message: "User exists".to_owned(),
            status_code: StatusCode::CONFLICT,
            error_code: Some(40),
        });
    }

    let user_model = user::ActiveModel {
        name: Set(user_data.name.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(user_data.password.to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    user_model.insert(&db).await.map_err(|err| APIError {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(50),
    })?;

    Ok(())
}

pub async fn login_user_post(
    axum::Extension(db): axum::Extension<DatabaseConnection>,
    Json(user_data): Json<LoginUserModel>,
) -> Result<Json<UserModel>, APIError> {
    // let db: DatabaseConnection =
    //     Database::connect("postgres://postgres:password@localhost:5432/webapi")
    //         .await
    //         .unwrap();

    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(user_data.email))
                .add(entity::user::Column::Password.eq(user_data.password)),
        )
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
        })?;

    let data = UserModel {
        name: user.name,
        email: user.email,
        password: user.password,
        uuid: user.uuid,
        created_at: user.created_at,
    };

    Ok(Json(data))
}

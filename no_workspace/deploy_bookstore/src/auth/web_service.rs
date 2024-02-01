use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use super::{auth_layers::ValidUser, db::{self, User}};


#[derive(Deserialize, Serialize, Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum LoginResponse {
    Success { token: String },
    Failure { reason: String },
}

pub async fn do_login(
    Extension(db_pool): Extension<db::AuthDb>,
    login_request: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    match db::login(
        db_pool.clone(),
        &login_request.username,
        &login_request.password,
    )
    .await
    {
        Ok(Some(user_id)) => {
            let token = db::add_token(db_pool.clone(), user_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(LoginResponse::Success { token }))
        }
        Ok(None) => Ok(Json(LoginResponse::Failure {
            reason: "Invalid username or password".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_users(
    Extension(db_pool): Extension<db::AuthDb>,
    Extension(_valid_user): Extension<ValidUser>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = db::get_all_users(db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn get_user(
    Extension(db_pool): Extension<db::AuthDb>,
    Extension(_valid_user): Extension<ValidUser>,
    path: axum::extract::Path<i32>,
) -> Result<Json<Option<User>>, StatusCode> {
    let user = db::get_user(db_pool, path.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}

pub async fn delete_user(
    Extension(db_pool): Extension<db::AuthDb>,
    Extension(_valid_user): Extension<ValidUser>,
    path: axum::extract::Path<i32>,
) -> Result<StatusCode, StatusCode> {
    db::delete_user(db_pool, path.0)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn update_user(
    Extension(db_pool): Extension<db::AuthDb>,
    Extension(_valid_user): Extension<ValidUser>,
    path: axum::extract::Path<i32>,
    update: Json<User>,
) -> Result<StatusCode, StatusCode> {
    db::update_user(db_pool, path.0, &update)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn add_user(
    Extension(db_pool): Extension<db::AuthDb>,
    Extension(_valid_user): Extension<ValidUser>,
    new_user: Json<User>,
) -> Result<StatusCode, StatusCode> {
    db::add_user(db_pool, &new_user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
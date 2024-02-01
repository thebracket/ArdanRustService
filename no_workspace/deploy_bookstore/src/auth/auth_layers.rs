use axum::{
    extract::Request, http::{HeaderMap, StatusCode}, middleware::Next, response::IntoResponse, Extension
};
use super::db;

#[derive(Clone, Copy, Debug)]
pub struct ValidUser(pub i32);

pub async fn require_token(
    Extension(db_pool): Extension<db::AuthDb>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth_header) = headers.get("Token") {
        let token = auth_header.to_str().map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "invalid header".to_string(),
            )
        })?;

        if let Some(user_id) = db::get_user_id_from_token(db_pool, token)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database error".to_string(),
                )
            })?
        {
            req.extensions_mut().insert(ValidUser(user_id));
            return Ok(next.run(req).await);
        }
    }

    Err((StatusCode::UNAUTHORIZED, "invalid header".to_string()))
}

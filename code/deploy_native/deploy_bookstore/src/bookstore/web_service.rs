use axum::{http::StatusCode, Extension, Json};
use super::db::{self, Book, StoreDb};

pub async fn all_books(
    Extension(db_pool): Extension<StoreDb>
) -> Result<Json<Vec<Book>>, StatusCode> {
    let books = db::list_books(db_pool).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(books))
}

pub async fn get_book(
    Extension(db_pool): Extension<StoreDb>,
    path: axum::extract::Path<i32>
) -> Result<Json<Book>, StatusCode> {
    let book = db::get_book(db_pool, path.0).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(book))
}

pub async fn delete_book(
    Extension(db_pool): Extension<StoreDb>,
    path: axum::extract::Path<i32>
) -> Result<StatusCode, StatusCode> {
    db::delete_book(db_pool, path.0).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn add_book(
    Extension(db_pool): Extension<StoreDb>,
    Json(book): Json<Book>
) -> Result<StatusCode, StatusCode> {
    db::add_book(db_pool, book.title, book.author).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn update_book(
    Extension(db_pool): Extension<StoreDb>,
    path: axum::extract::Path<i32>,
    Json(book): Json<Book>
) -> Result<StatusCode, StatusCode> {
    db::update_book(db_pool, path.0, book.title, book.author).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}
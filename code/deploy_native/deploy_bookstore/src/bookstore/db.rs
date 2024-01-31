use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqliteConnectOptions};

#[derive(Clone)]
pub struct StoreDb(pub sqlx::SqlitePool);

pub async fn get_connection_pool(filename: &str) -> Result<StoreDb> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    let connection_pool = sqlx::SqlitePool::connect_with(options)
        .await?;
    Ok(StoreDb(connection_pool))
}

pub async fn perform_migrations(db_pool: StoreDb) -> Result<()> {
    sqlx::migrate!("src/bookstore/migrations")
        .run(&db_pool.0)
        .await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}

pub async fn list_books(db_pool: StoreDb) -> Result<Vec<Book>> {
    let books = sqlx::query_as::<_, Book>("SELECT * FROM books")
        .fetch_all(&db_pool.0)
        .await?;
    Ok(books)
}

pub async fn get_book(db_pool: StoreDb, id: i32) -> Result<Book> {
    let book = sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = ?")
        .bind(id)
        .fetch_one(&db_pool.0)
        .await?;
    Ok(book)
}

pub async fn delete_book(db_pool: StoreDb, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM books WHERE id = ?")
        .bind(id)
        .execute(&db_pool.0)
        .await?;
    Ok(())
}

pub async fn add_book(db_pool: StoreDb, title: String, author: String) -> Result<()> {
    sqlx::query("INSERT INTO books (title, author) VALUES (?, ?)")
        .bind(title)
        .bind(author)
        .execute(&db_pool.0)
        .await?;
    Ok(())
}

pub async fn update_book(db_pool: StoreDb, id: i32, title: String, author: String) -> Result<()> {
    sqlx::query("UPDATE books SET title = ?, author = ? WHERE id = ?")
        .bind(title)
        .bind(author)
        .bind(id)
        .execute(&db_pool.0)
        .await?;
    Ok(())
}
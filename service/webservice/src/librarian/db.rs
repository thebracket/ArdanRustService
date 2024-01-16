use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Row, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;
use tracing::info;
use crate::librarian::LibrarianConfig;

#[derive(Clone)]
pub struct LibrarianDb(pub Pool<Sqlite>);

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Book {
    pub id: i32,
    pub author: String,
    pub title: String,
}

pub async fn setup_database(settings: &LibrarianConfig) -> anyhow::Result<LibrarianDb> {
    info!("Binding to database");
    let db = LibrarianDb(
        SqlitePoolOptions::new()
            .connect(&settings.database_url)
            .await?,
    );

    info!("Running Librarian Database Migrations");
    sqlx::migrate!("./migrations").run(&db.0).await?;
    Ok(db)
}

pub async fn all_books(connection: &LibrarianDb) -> anyhow::Result<Vec<Book>> {
    let rows = sqlx::query_as::<_, Book>("SELECT * FROM books ORDER BY author, title")
        .fetch_all(&connection.0)
        .await?;
    Ok(rows)
}

pub async fn book_by_id(connection: &LibrarianDb, id: i32) -> anyhow::Result<Book> {
    let book = sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id=?")
        .bind(id)
        .fetch_one(&connection.0)
        .await?;
    Ok(book)
}

pub async fn add_book(connection: &LibrarianDb, book: Book) -> anyhow::Result<i32> {
    let new_id = sqlx::query("INSERT INTO books (author, title) VALUES (?, ?) RETURNING id")
        .bind(&book.author)
        .bind(&book.title)
        .fetch_one(&connection.0)
        .await?
        .try_get(0)?;

    Ok(new_id)
}

pub async fn update_book(connection: &LibrarianDb, book: &Book) -> anyhow::Result<()> {
    sqlx::query("UPDATE books SET author=?, title=? WHERE id=?")
        .bind(&book.author)
        .bind(&book.title)
        .bind(book.id)
        .execute(&connection.0)
        .await?;

    Ok(())
}

pub async fn delete_book(connection: &LibrarianDb, id: i32) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM books WHERE id=?")
        .bind(id)
        .execute(&connection.0)
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    async fn get_db() -> LibrarianDb {
        let db = LibrarianDb(
            SqlitePoolOptions::new()
                .connect("sqlite::memory:")
                .await
                .unwrap()
        );

        info!("Running Librarian Database Migrations");
        sqlx::migrate!("./migrations").run(&db.0).await.unwrap();

        db
    }

    #[sqlx::test]
    async fn get_all() {
        let db =get_db().await;
        let all_rows = all_books(&db).await.unwrap();
        assert!(!all_rows.is_empty())
    }

    #[sqlx::test]
    async fn get_one() {
        let db =get_db().await;
        let book = book_by_id(&db, 1).await.unwrap();
        assert_eq!(1, book.id);
        assert_eq!("Hands-on Rust", book.title);
        assert_eq!("Wolverson, Herbert", book.author);
    }

    #[sqlx::test]
    async fn test_create() {
        let db =get_db().await;
        let new_id = add_book(&db, Book{ id: -1, title: "Test Book".to_string(),  author: "Test Author".to_string()}).await.unwrap();
        let new_book = book_by_id(&db, new_id).await.unwrap();
        assert_eq!(new_id, new_book.id);
        assert_eq!("Test Book", new_book.title);
        assert_eq!("Test Author", new_book.author);
    }

    #[sqlx::test]
    async fn test_update() {
        let db =get_db().await;
        let mut book = book_by_id(&db, 2).await.unwrap();
        book.title = "Updated Book".to_string();
        update_book(&db, &book).await.unwrap();
        let updated_book = book_by_id(&db, 2).await.unwrap();
        assert_eq!("Updated Book", updated_book.title);
    }

    #[sqlx::test]
    async fn test_delete() {
        let db =get_db().await;
        let new_id = add_book(&db, Book { id: -1, title: "DeleteMe".to_string(), author: "Test Author".to_string()}).await.unwrap();
        let _new_book = book_by_id(&db, new_id).await.unwrap();
        delete_book(&db, new_id).await.unwrap();
        let all_books = all_books(&db).await.unwrap();
        assert!(!all_books.iter().any(|b| b.title == "DeleteMe"));
    }
}
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqliteConnectOptions, Row};

#[derive(Clone)]
pub struct AuthDb(pub sqlx::SqlitePool);

pub async fn get_connection_pool(filename: &str) -> Result<AuthDb> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    let connection_pool = sqlx::SqlitePool::connect_with(options)
        .await?;
    Ok(AuthDb(connection_pool))
}

pub async fn perform_migrations(db_pool: AuthDb) -> Result<()> {
    sqlx::migrate!("src/auth/migrations")
        .run(&db_pool.0)
        .await?;
    Ok(())
}

pub async fn login(db_pool: AuthDb, username: &str, password: &str) -> Result<Option<i32>> {
    let user_id = sqlx::query("SELECT id FROM users WHERE username = ? AND password = ?")
        .bind(username)
        .bind(password)
        .fetch_optional(&db_pool.0)
        .await?
        .map(|row| row.get::<i32, _>(0) as i32);

    Ok(user_id)
}

pub async fn add_token(db_pool: AuthDb, user_id: i32) -> Result<String> {
    let new_token = uuid::Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO tokens (user_id, token) VALUES (?, ?)")
        .bind(user_id)
        .bind(&new_token)
        .execute(&db_pool.0)
        .await?;

    Ok(new_token)
}

pub async fn get_user_id_from_token(db_pool: AuthDb, token: &str) -> Result<Option<i32>> {
    let user_id = sqlx::query("SELECT user_id FROM tokens WHERE token = ?")
        .bind(token)
        .fetch_optional(&db_pool.0)
        .await?
        .map(|row| row.get::<i32, _>(0) as i32);

    Ok(user_id)
}

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct User {
    id: i32,
    username: String,
    password: String,
}

pub async fn get_all_users(db_pool: AuthDb) -> Result<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT id, username, password FROM users")
        .fetch_all(&db_pool.0)
        .await?;

    Ok(users)
}

pub async fn get_user(db_pool: AuthDb, user_id: i32) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT id, username, password FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(&db_pool.0)
        .await?;

    Ok(user)
}

pub async fn delete_user(db_pool: AuthDb, user_id: i32) -> Result<()> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(user_id)
        .execute(&db_pool.0)
        .await?;

    Ok(())
}

pub async fn update_user(db_pool: AuthDb, user_id: i32, user: &User) -> Result<()> {
    sqlx::query("UPDATE users SET username = ?, password = ? WHERE id = ?")
        .bind(&user.username)
        .bind(&user.password)
        .bind(user_id)
        .execute(&db_pool.0)
        .await?;

    Ok(())
}

pub async fn add_user(db_pool: AuthDb, user: &User) -> Result<()> {
    sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(&user.username)
        .bind(&user.password)
        .execute(&db_pool.0)
        .await?;

    Ok(())
}
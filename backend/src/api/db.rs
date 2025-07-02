use actix_web::cookie::time::Result;
use serde::{Serialize, de};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, mysql::MySqlPool};

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    id: u32,
    password: String,
    email: String,
    created_at: Option<DateTime<Utc>>,
    username: String,
}

#[derive(Debug, FromRow)]
pub struct Todo {
    id: u32,
    uid: u32,
    title: String,
    descr: String,
    status: String,
    dtime: DateTime<Utc>,
}

pub async fn get_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, password, email, created_at, username,
        FROM user
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn get_user(pool: &MySqlPool, id: u32) -> Result<Vec<User>, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM user WHERE id = ?", id)
        .fetch_all(pool)
        .await?;
    Ok(user)
}

pub async fn add_user(
    pool: &MySqlPool,
    password: &str,
    email: &str,
    username: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query_as!(
        r#"
        INSERT INTO user (email, password, username)
        VALUES (?, ?, ?, ?)
        "#,
        email,
        password,
        username,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_user_todo(pool: &MySqlPool, user_id: i32) -> Result<Vec<Todo>, sqlx::Error> {
    let Utodo = sqlx::query_as!(Todo, "SELLECT * FROM todo WHERE user_id = ?", user_id)
        .fetch_all(pool)
        .await?;
    Ok(Utodo)
}

pub async fn get_todo(pool: &MySqlPool, id: u32) -> Result<Vec<Todo>, sqlx::Error> {
    let todo = sqlx::query_as!(Todo, "SELLECT * FROM todo WHERE id = ?", id)
        .fetch_all(pool)
        .await?;
    Ok(todo)
}

pub async fn add_todo(
    pool: &MySqlPool,
    title: &str,
    descr: &str,
    status: &str,
    d_time: DateTime<Utc>,
) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as!(
        r#"INSERT INTO todo (title, description, status, datetime) VALUES(?, ?, ?, ?)"#,
        title,
        descr,
        status,
        d_time,
    )
    .execute(pool)
    .await?;

    OK(())
}

pub async fn update_user(
    pool: &MySqlPool,
    id: u32,
    password: &str,
    email: &str,
    username: &str,
) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        r#"UPDATE \`user\`
    SET email = ?, password = ?, username = ?
    WHERE id = ?"#,
        password,
        username,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_todo(
    pool: &MySqlPool,
    id: u32,
    user_id: i32,
    title: &str,
    descr: &str,
    status: &str,
    d_time: DateTime<Utc>,
) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as!(
        r#"UPDATE \`todo\`
    SET title = ?, description = ?, due_time = ?, user_id = ?, status = ?
    WHERE id = ?"#,
        title,
        descr,
        d_time,
        uid,
        status,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_user(pool: &MySqlPool, id: u32) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(r#"DELETE FROM user WHERE id = ?"#, id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_todo(pool: &MySqlPool, id: u32) {
    sqlx::query_as!(r#"DELETE FROM todo WHERE id = ?"#, id)
        .execute(pool)
        .await?;
    Ok(())
}

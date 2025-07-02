use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, mysql::MySqlPool};

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    id: u32,
    password: String,
    email: String,
    created_at: Option<DateTime<Utc>>,
    first_name: String,
    last_name: String,
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
        SELECT id, password, email, created_at, first_name, last_name
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
    first_name: &str,
    last_name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO user (email, password, first_name, last_name)
        VALUES (?, ?, ?, ?)
        "#,
        email,
        password,
        first_name,
        last_name
    )
    .execute(pool)
    .await?;

    Ok(())
}

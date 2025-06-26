use super::db;
use actix_web::{HttpResponse, Responder, get, web};
use sqlx::mysql::MySqlPool;

#[get("/users")]
pub async fn show_users(pool: web::Data<MySqlPool>) -> impl Responder {
    match db::get_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            eprintln!("Failed to fetch users: {}", err);
            HttpResponse::InternalServerError().body("Error fetching users")
        }
    }
}

#[get("/user/{id}")]
pub async fn show_user(pool: web::Data<MySqlPool>, id: web::Path<u32>) -> impl Responder {
    let wid = id.into_inner();
    match db::get_user(pool.get_ref(), wid).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            eprintln!("Failed to fetch users: {}", err);
            HttpResponse::InternalServerError().body("Error fetching users")
        }
    }
}

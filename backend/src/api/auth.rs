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


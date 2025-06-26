pub mod db;
mod music;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use sqlx::mysql::MySqlPoolOptions;
use std::{env, path::PathBuf, sync::Mutex};

const MUSIC_DIRECTORY_ENV_VAR: &str = "MUSIC_DIR";

#[get("/")]
async fn default() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[allow(clippy::manual_strip, clippy::io_other_error)]
pub async fn api() -> std::io::Result<()> {
    let music_dir_str =
        std::env::var(MUSIC_DIRECTORY_ENV_VAR).unwrap_or_else(|_| String::from("./music"));
    let music_dir = PathBuf::from(music_dir_str);
    let tracks_map = match music::load_music_files(&music_dir).await {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error loading music files: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };
    let app_state = web::Data::new(music::AppState::new(
        music_dir.clone(),
        Mutex::new(tracks_map),
    ));
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("database_url must be set");
    let pool = match MySqlPoolOptions::new().connect(&database_url).await {
        Ok(p) => web::Data::new(p),
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    };
    HttpServer::new(move || {
        App::new()
            .service(default)
            .service(
                web::scope("/music")
                    .wrap(
                        Cors::permissive()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                            .max_age(3600),
                    )
                    .app_data(app_state.clone())
                    .service(music::list)
                    .service(music::stream),
            )
            .service(
                web::scope("/auth")
                    .wrap(
                        Cors::permissive()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                            .max_age(3600),
                    )
                    .app_data(pool.clone())
                    .service(auth::show_users),
            )
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}

mod music;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use std::{path::PathBuf, sync::Mutex};

const MUSIC_DIRECTORY_ENV_VAR: &str = "MUSIC_DIR";

#[get("/")]
async fn default() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
#[allow(clippy::manual_strip, clippy::io_other_error)]
async fn main() -> std::io::Result<()> {
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

    HttpServer::new(move || {
        App::new().service(
            web::scope("/music")
                .app_data(app_state.clone())
                .service(music::list)
                .service(music::stream),
        )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

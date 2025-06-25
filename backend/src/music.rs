use actix_web::{HttpResponse, Responder, get, http::header, web};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::SeekFrom,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MusicInfo {
    file: String,
    path: String,
    mime: String,
}

#[derive(Debug)]
pub struct AppState {
    music_dir: PathBuf,
    tracks: Mutex<HashMap<String, MusicInfo>>,
}

impl AppState {
    pub fn new(music_dir: PathBuf, tracks: Mutex<HashMap<String, MusicInfo>>) -> Self {
        AppState { music_dir, tracks }
    }
}

pub async fn load_music_files(music_dir: &Path) -> Result<HashMap<String, MusicInfo>, String> {
    let mut tracks = HashMap::new();
    println!("Scanning music directory: {:?}", music_dir);

    // Ensure the music directory exists.
    if !music_dir.exists() {
        return Err(format!("Music directory does not exist: {:?}", music_dir));
    }
    if !music_dir.is_dir() {
        return Err(format!(
            "Music directory is not a directory: {:?}",
            music_dir
        ));
    }

    for entry in WalkDir::new(music_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            // Guess the MIME type based on file extension.
            let mime_type = mime_guess::from_path(path)
                .first_or_text_plain()
                .to_string();

            // Check if it's an audio file based on common MIME types.
            if mime_type.starts_with("audio/") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    let relative_path = path
                        .strip_prefix(music_dir)
                        .unwrap_or(path) // Fallback if prefix fails (shouldn't for files within music_dir)
                        .to_string_lossy()
                        .into_owned(); // Convert Cow to owned String

                    let track_info = MusicInfo {
                        file: filename.to_string(),
                        path: relative_path.clone(), // Use relative path for streaming URL
                        mime: mime_type,
                    };
                    tracks.insert(relative_path, track_info); // Store by relative path
                    println!("Found music file: {}", filename);
                }
            }
        }
    }
    println!("Finished scanning. Found {} music files.", tracks.len());
    Ok(tracks)
}

#[get("/")]
pub async fn list(data: web::Data<AppState>) -> impl Responder {
    let tracks = data.tracks.lock().unwrap();
    let track_list: Vec<&MusicInfo> = tracks.values().collect();
    HttpResponse::Ok().json(track_list)
}

#[get("/{path:.*}")]
#[allow(clippy::manual_strip, clippy::await_holding_lock)]
pub async fn stream(
    path: web::Path<String>,
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let req_path = path.into_inner();
    let tracks = data.tracks.lock().unwrap();

    let track_info = tracks
        .get(&req_path)
        .ok_or_else(|| actix_web::error::ErrorNotFound(format!("Track not found: {}", req_path)))?;
    let file_path = data.music_dir.join(&track_info.path);
    let file_size;
    let mut file = match File::open(&file_path).await {
        Ok(f) => {
            file_size = f.metadata().await?.len();
            f
        }
        Err(e) => {
            eprintln!("Failed to open file {}: {}", file_path.display(), e);
            return Err(actix_web::error::ErrorInternalServerError(
                "Failed to open file",
            ));
        }
    };

    let range_header = req.headers().get(header::RANGE);

    if let Some(range_header_value) = range_header {
        let range_str = range_header_value
            .to_str()
            .map_err(|_| actix_web::error::ErrorBadRequest("Invalid Range header"))?;

        if range_str.starts_with("bytes=") {
            let parts: Vec<&str> = range_str["bytes=".len()..].split('-').collect();
            if parts.len() == 2 {
                let start_byte: u64 = parts[0].parse().unwrap_or(0);
                let end_byte_option: Option<u64> = parts[1].parse().ok();

                let end_byte = end_byte_option.unwrap_or(file_size - 1);

                if start_byte >= file_size || start_byte > end_byte {
                    return Ok(HttpResponse::RangeNotSatisfiable()
                        .insert_header(header::ContentRange(
                            format!("bytes */{}", file_size).parse().unwrap(),
                        ))
                        .finish());
                }

                file.seek(SeekFrom::Start(start_byte)).await?;

                let content_length = (end_byte - start_byte + 1).min(file_size - start_byte);
                let content_range = format!(
                    "bytes {}-{}/{}",
                    start_byte,
                    start_byte + content_length - 1,
                    file_size
                );

                println!("Serving Range {}", content_range);

                let stream = tokio_util::io::ReaderStream::new(file.take(content_length));

                return Ok(HttpResponse::PartialContent()
                    .insert_header(header::ContentType(track_info.mime.parse().unwrap()))
                    .insert_header(header::ContentLength(content_length as usize))
                    .insert_header(header::ContentRange(content_range.parse().unwrap()))
                    .streaming(stream));
            }
        }
    }

    let stream = tokio_util::io::ReaderStream::new(file);

    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType(track_info.mime.parse().unwrap()))
        .insert_header(header::ContentLength(file_size as usize))
        .streaming(stream))
}

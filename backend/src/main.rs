use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn default() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(default))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}

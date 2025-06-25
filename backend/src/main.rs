mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    api::api().await
}

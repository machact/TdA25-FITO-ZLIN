use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use actix_files::NamedFile;
use log::info;
mod api;

async fn index() -> impl Responder {
    NamedFile::open("static/index.html")
}

async fn not_found() -> impl Responder {
    NamedFile::open("static/404.html")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    info!("Starting...");

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .route("/", web::get().to(index))
            .route("/api", web::get().to(api::hello_api))
            .default_service(web::route().to(not_found))
            .wrap(logger)
    }).bind(format!("0.0.0.0:{port}"))?.run().await
}

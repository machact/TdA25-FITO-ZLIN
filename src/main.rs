use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use actix_files::{Files, NamedFile};
use log::info;
mod api;


async fn not_found() -> impl Responder {
    NamedFile::open("static/404.html")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::from_filename(".env");
    env_logger::init();

    info!("Starting...");

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .route("/api", web::get().to(api::hello_api))
            .default_service(web::route().to(not_found))
            .service(Files::new("/", "static/root/").index_file("index.html"))
            //.service(Files::new("/static", "static"))
            .wrap(logger)
    }).bind(format!("0.0.0.0:{port}"))?.run().await
}

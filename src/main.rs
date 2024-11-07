use actix_web::{middleware, web, App, HttpServer, Responder};
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
        let logger = middleware::Logger::default();
        App::new()
            .wrap(logger)
            .default_service(web::route().to(not_found))
            .route("/api", web::get().to(api::hello_api))
            .service(
                Files::new("/game/", "static/game/")
                    .index_file("index.html")
                    .redirect_to_slash_directory()
            )
            .service(Files::new("/", "static/root/").index_file("index.html"))
    }).bind(format!("0.0.0.0:{port}"))?.run().await
}

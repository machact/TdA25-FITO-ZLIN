use actix_web::{HttpServer, App, web, Responder, middleware::Logger};
use log::info;
mod api;

async fn status() -> impl Responder {
    "Hello TdA"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    info!("Starting...");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .route("/", web::get().to(status))
            .wrap(logger)
    }).bind("127.0.0.1:8080")?.run().await
}

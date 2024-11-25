use actix_web::{http::StatusCode, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files::{Files, NamedFile};
use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use log::{info, error};
use std::env;

pub mod api;
pub mod api_config;

async fn not_found(req: HttpRequest) -> impl Responder {
    match NamedFile::open("static/404.html") {
        Ok(file) => {
            let mut response = file.into_response(&req);
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        },
        Err(_) => HttpResponse::NotFound().body("404 not found"),
    }
}

async fn setup_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS games (
                uuid TEXT PRIMARY KEY,
                createdAt TEXT NOT NULL,
                uploadedAt TEXT NOT NULL,
                name TEXT NOT NULL,
                difficulty TEXT NOT NULL,
                gameState TEXT NOT NULL,
                board TEXT NOT NULL
            );
        "#
    )
        .execute(pool)
        .await?;
    
    Ok(())
}




#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(_) = dotenvy::from_filename(".env") {
        error!("Couldn't find .env file.")
    }
    env_logger::init();

    let database_path = "sqlite:app.db";

    let options = SqliteConnectOptions::new()
        .filename(database_path)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    setup_db(&pool).await?;

    info!("Starting...");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(move || {
        let logger = middleware::Logger::default();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(logger)
            .configure(api_config::api_config)
            .service(
                Files::new("/game/", "static/game/")
                    .index_file("game.html")
                    .redirect_to_slash_directory()
            )
            .service(Files::new("/", "static/root/").index_file("index.html"))
            .default_service(web::route().to(not_found))
    }).bind(format!("0.0.0.0:{port}"))?.run().await?;

    Ok(())
}

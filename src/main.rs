use actix_web::{middleware, web, App, HttpServer, Responder};
use actix_files::{Files, NamedFile};
use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use log::{info, error};
use std::env;

mod api;


async fn not_found() -> impl Responder {
    NamedFile::open("static/404.html")
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
    info!("DATABASE_URL: {}", database_path);

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
            .default_service(web::route().to(not_found))
            .route("/api/v1/games", web::get().to(api::games_get))
            .route("/api/v1/games", web::post().to(api::games_post))
            .route("/api", web::get().to(api::hello_api))
            .service(
                Files::new("/game/", "static/game/")
                    .index_file("game.html")
                    .redirect_to_slash_directory()
            )
            .service(Files::new("/", "static/root/").index_file("index.html"))
    }).bind(format!("0.0.0.0:{port}"))?.run().await?;

    Ok(())
}

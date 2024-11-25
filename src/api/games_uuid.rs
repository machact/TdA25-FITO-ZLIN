use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use super::api_utils::{GameDatabase, Game};

use super::api_utils::GameError;

pub async fn get(pool: web::Data<SqlitePool>, uuid: web::Path<String>) -> Result<HttpResponse, GameError> {
    let game_db: GameDatabase = sqlx::query_as("SELECT * FROM games WHERE uuid = ?")
        .bind(uuid.as_ref())
        .fetch_one(pool.get_ref())
        .await
        .map_err(|err| GameError::NotFound(err.to_string()))?;
    let game: Game = game_db.try_into()?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(game)
    )
}

pub async fn delete(pool: web::Data<SqlitePool>, uuid: web::Path<String>) -> Result<HttpResponse, GameError> {
    let result = sqlx::query("DELETE FROM games WHERE uuid = ?")
        .bind(uuid.as_ref())
        .execute(pool.get_ref())
        .await
        .map_err(|err| GameError::NotFound(err.to_string()))?;

    if result.rows_affected() != 1 {
        return Ok(HttpResponse::NotFound()
            .content_type(ContentType::plaintext())
            .body("Not found")
        );
    }
    
    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Deleted")
    )
}

pub async fn put(pool: web::Data<SqlitePool>, uuid: web::Path<String>) -> Result<HttpResponse, GameError> {

    todo!()
}
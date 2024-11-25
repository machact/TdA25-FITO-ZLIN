use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::SqlitePool;
use crate::api::api_utils::is_board_valid;

use super::api_utils::{CreateUpdateGame, Game, GameDatabase};

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
    let result: SqliteQueryResult = sqlx::query("DELETE FROM games WHERE uuid = ?")
        .bind(uuid.as_ref())
        .execute(pool.get_ref())
        .await
        .map_err(|err| GameError::NotFound(err.to_string()))?;

    if result.rows_affected() != 1 {
        return Err(GameError::NotFound(format!("Game with uuid {} not found", uuid)))
    }
    
    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Deleted")
    )
}

pub async fn put(pool: web::Data<SqlitePool>, game_data: web::Json<CreateUpdateGame>, uuid: web::Path<String>) -> Result<HttpResponse, GameError> {
    let update_time = Utc::now().to_rfc3339();

    is_board_valid(&game_data.board).await?;
    let board_json_str = serde_json::to_string(&game_data.board)?;

    let result: SqliteQueryResult = sqlx::query(
        r#"
            UPDATE games
            SET updatedAt = ?, name = ?, difficulty = ?, gameState = ?, board = ?
            WHERE uuid = ?
        "#
    ).bind(update_time)
        .bind(&game_data.name)
        .bind(&game_data.difficulty)
        .bind("unknown") // TODO
        .bind(&board_json_str)
        .bind(uuid.as_ref())
        .execute(pool.get_ref())
        .await?;

    if result.rows_affected() != 1 {
        return Err(GameError::NotFound(format!("Game with uuid {} not found", uuid)))
    }
        
    get(pool, uuid).await
}
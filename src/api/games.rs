use actix_web::{web, HttpResponse};
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;
use super::api_utils::{CreateGameRequest, Game, GameDatabase, GameError, is_board_valid};



pub async fn post(pool: web::Data<SqlitePool>, game_data: web::Json<CreateGameRequest>) -> Result<HttpResponse, GameError> {
    let uuid = Uuid::new_v4().to_string();
    let current_time = Utc::now().to_rfc3339();

    is_board_valid(&game_data.board).await?;

    let board_json_str = serde_json::to_string(&game_data.board)?;


    sqlx::query(
        r#"
            INSERT INTO games (
                uuid,
                createdAt,
                uploadedAt,
                name,
                difficulty,
                gameState,
                board
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#
    ).bind(&uuid)
        .bind(&current_time)
        .bind(&current_time)
        .bind(&game_data.name)
        .bind(&game_data.difficulty)
        .bind("unknown")
        .bind(&board_json_str)
        .execute(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created()
        .content_type("application/json")
        .json(json!({
            "uuid": uuid,
            "createdAt": current_time,
            "updatedAt": current_time,
            "name": game_data.name,
            "difficulty": game_data.difficulty,
            "game_state": "unknown",
            "board": game_data.board
        })))

}


pub async fn get(pool: web::Data<SqlitePool>) -> Result<HttpResponse, GameError> {
    let games_database: Vec<GameDatabase> = sqlx::query_as(
        r#"
        SELECT *
        FROM games
        "#
    )
    .fetch_all(pool.get_ref())
    .await?;

    let games: Result<Vec<Game>, GameError> = games_database.into_iter()
        .map(|game_database| {
            game_database.try_into().map_err(|e: serde_json::Error| GameError::JsonConvertionError(e))
        })
        .collect();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(games?)
    )  
}


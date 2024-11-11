use actix_web::{web, HttpResponse};
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;
use super::types::{CreateGameRequest, Game, GameDatabase, GameError};



pub async fn post(pool: web::Data<SqlitePool>, game_data: web::Json<CreateGameRequest >) -> Result<HttpResponse, GameError> {
    let uuid = Uuid::new_v4().to_string();
    let current_time = Utc::now().to_rfc3339();

    if game_data.board.len() != 15 {
        return Err(GameError::InvalidBoard("Board must be 15x15".to_string()));
    }
    let mut xs = 0;
    let mut os = 0;
    for row in &game_data.board {
        if row.len() != 15 {
            return Err(GameError::InvalidBoard("Board must be 15x15".to_string()));
        }
        for i in row {
            match &**i {
                "" => (),
                "X" => xs += 1,
                "O" => os += 1,
                _ => {
                    return Err(GameError::InvalidBoard(r#"Board must only contain "", "X" or "O\"#.to_string()));
                }
            }
        }
    }
    if os > xs || xs > os + 1 {
        return Err(GameError::InvalidBoard("Invalid board".to_string()));
    }

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
    let games_database = sqlx::query_as::<_, GameDatabase>(
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

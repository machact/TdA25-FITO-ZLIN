use actix_web::{web, Responder, HttpResponse};
use serde_json::json;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqlitePool};
use log::error;
use uuid::Uuid;
use chrono::Utc;


#[derive(Deserialize, Serialize, FromRow)]
#[sqlx(rename_all = "camelCase")]
struct Game {
    uuid: String,

    #[serde(rename = "createdAt")]
    created_at: String,

    #[serde(rename = "uploadedAt")]
    uploaded_at: String,

    name: String,
    difficulty: String,

    #[serde(rename = "gameState")]
    game_state: String,

    board: String,
}


#[derive(Deserialize, Serialize)]
pub struct CreateGameRequest {
    name: String,
    difficulty: String,
    board: Vec<Vec<String>>,
}




pub async fn games_get(pool: web::Data<SqlitePool>) -> impl Responder {
    let games_result = sqlx::query_as::<_, Game>(
        r#"
        SELECT *
        FROM games
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match games_result {
        Ok(games) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(games)
        }
        Err(err) => {
            error!("Database error: {}", err);
            HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(json!({
                    "error": "Failed to fetch games",
                    "details": err.to_string()
                }))
        }
    }
}

pub async fn games_post(pool: web::Data<SqlitePool>, game_data: web::Json<CreateGameRequest >) -> impl Responder {
    let uuid = Uuid::new_v4().to_string();
    let current_time = Utc::now().to_rfc3339();

    let board_json = match serde_json::to_string(&game_data.board) {
        Ok(json) => json,
        Err(e) => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid board data",
                "details": e.to_string()
            }));
        }
    };

    let result = sqlx::query(
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
        .bind(&board_json)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => {
            HttpResponse::Created()
                .content_type("application/json")
                .json(json!({
                    "uuid": uuid,
                    "createdAt": current_time,
                    "updatedAt": current_time,
                    "name": game_data.name,
                    "difficulty": game_data.difficulty,
                    "game_state": "unknown",
                    "board": game_data.board
                }))
        }
        Err(err) => {
            error!("Database error: {}", err);
            HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(json!({
                    "error": "Failed to create game",
                    "details": err.to_string()
                }))
        }
    }
}


pub async fn hello_api() -> impl Responder {
    web::Json(json!({"organization": "Student Cyber Games"}))
}
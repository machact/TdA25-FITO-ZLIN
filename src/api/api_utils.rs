use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use log::{error, warn};
use sqlx::prelude::FromRow;



#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub uuid: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "uploadedAt")]
    pub uploaded_at: String,

    pub name: String,
    pub difficulty: String,

    #[serde(rename = "gameState")]
    pub game_state: String,

    pub board: Vec<Vec<String>>, // It's string instead of char because json
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct GameDatabase {
    pub uuid: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "uploadedAt")]
    pub uploaded_at: String,

    pub name: String,
    pub difficulty: String,

    #[serde(rename = "gameState")]
    pub game_state: String,

    pub board: String,
}

impl TryFrom<Game> for GameDatabase {
    type Error = serde_json::Error;
    fn try_from(game: Game) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: game.uuid,
            created_at: game.created_at,
            uploaded_at: game.uploaded_at,
            name: game.name,
            difficulty: game.difficulty,
            game_state: game.game_state,
            board: serde_json::to_string(&game.board)?
        })
    }
}

impl TryFrom<GameDatabase> for Game {
    type Error = serde_json::Error;
    fn try_from(game: GameDatabase) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: game.uuid,
            created_at: game.created_at,
            uploaded_at: game.uploaded_at,
            name: game.name,
            difficulty: game.difficulty,
            game_state: game.game_state,
            board: serde_json::from_str(&game.board)?
        })
    }
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Json convertion error (400): {0}")]
    JsonConvertionError(#[from] serde_json::Error),

    #[error("Not found (404): {0}")]
    NotFound(String),

    #[error("Invalid board format (422): {0}")]
    InvalidBoard(String)
}

impl actix_web::ResponseError for GameError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GameError::DatabaseError(err) => {
                error!("{}", self);
                HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .json(json!({
                        "error": "Internal error",
                        "details": err.to_string()
                    }))
            }
            GameError::JsonConvertionError(err) => {
                warn!("{}", self);
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .json(json!({
                        "error": "Json convertion error",
                        "details": err.to_string()
                    }))
            }
            GameError::NotFound(err) => {
                warn!("{}", self);
                HttpResponse::NotFound()
                    .content_type("application/json")
                        .json(json!({
                            "error": "Not found",
                            "details": err.to_string()
                        }))
            }
            GameError::InvalidBoard(err) => {
                warn!("{}", self);
                HttpResponse::UnprocessableEntity()
                    .content_type("application/json")
                    .json(json!({
                        "error": "Invalid board",
                        "details": err.to_string()
                    }))
            }
        }
        
    }
}



#[derive(Deserialize, Serialize)]
pub struct CreateUpdateGame {
    pub name: String,
    pub difficulty: String,
    pub board: Vec<Vec<String>>,
}

pub async fn is_board_valid(board: &Vec<Vec<String>>) -> Result<(), GameError> {
    if board.len() != 15 {
        return Err(GameError::InvalidBoard("Board must be 15x15".to_string()));
    }
    let mut xs = 0;
    let mut os = 0;
    for row in board {
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

    Ok(())
}

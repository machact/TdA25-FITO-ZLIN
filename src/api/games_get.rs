use actix_web::{web, Responder, HttpResponse};
use sqlx::sqlite::SqlitePool;
use serde_json::json;
use log::error;
use crate::api::structs::*;


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

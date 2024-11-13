use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use super::api_utils::GameError;

pub async fn delete(pool: web::Data<SqlitePool>, uuid: web::Path<String>) -> Result<HttpResponse, GameError> {
    todo!()
}

pub async fn put(pool: web::Data<SqlitePool>, uuid: web::Path<String>) -> Result<HttpResponse, GameError> {
    todo!()
}
use actix_web::{web, Responder};
use serde_json::json;

pub mod api_utils;
pub mod games;
pub mod games_uuid;

pub async fn hello_api() -> impl Responder {
    web::Json(json!({"organization": "Student Cyber Games"}))
}
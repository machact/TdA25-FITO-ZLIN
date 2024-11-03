use actix_web::{web, Responder};
use serde_json::json;

pub async fn hello_api() -> impl Responder {
    web::Json(json!({"organization": "Student Cyber Games"}))
}
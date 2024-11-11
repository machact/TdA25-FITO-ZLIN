use actix_web::{web, Responder};
use serde_json::json;

pub mod types;
pub mod games;


pub async fn hello_api() -> impl Responder {
    web::Json(json!({"organization": "Student Cyber Games"}))
}
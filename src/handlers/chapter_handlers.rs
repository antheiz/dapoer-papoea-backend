use actix_web::{HttpResponse, Responder};
use crate::models::chapter::Chapter;
use crate::services::file_reader::read_json_file;

pub async fn get_chapters() -> impl Responder {
    match read_json_file::<Vec<Chapter>>("data/chapters.json") {
        Ok(chapters) => HttpResponse::Ok().json(chapters),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to read chapters data: {}", e)),
    }
}
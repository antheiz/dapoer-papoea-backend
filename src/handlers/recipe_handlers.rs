use actix_web::{HttpResponse, Responder};
use crate::models::recipe::Recipe;
use crate::services::file_reader::read_json_file;

pub async fn get_recipes() -> impl Responder {
    match read_json_file::<Vec<Recipe>>("data/recipes.json") {
        Ok(recipes) => HttpResponse::Ok().json(recipes),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to read recipes data: {}", e)),
    }
}
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// Data structure untuk Chapter (API Pertama)
#[derive(Serialize, Deserialize)]
struct Chapter {
    chapter: Option<String>,
    title: String,
    slug: String,
    #[serde(rename = "type")]
    type_field: String,
}

// Data structure untuk Recipe (API Kedua)
#[derive(Serialize, Deserialize)]
struct Ingredient {
    name: String,
    quantity: String,
}

#[derive(Serialize, Deserialize)]
struct Instruction {
    step: u32,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Recipe {
    title: String,
    slug: String,
    category: String,
    description: String,
    ingredients: Vec<Ingredient>,
    instructions: Vec<Instruction>,
}

// Helper function untuk membaca JSON dari file
fn read_json_file<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, String> {
    let path = PathBuf::from(path);
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

// Handler untuk GET request untuk mengambil chapters (API Pertama)
async fn get_chapters() -> impl Responder {
    match read_json_file::<Vec<Chapter>>("data/chapters.json") {
        Ok(chapters) => HttpResponse::Ok().json(chapters),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to read chapters data: {}", e)),
    }
}

// Handler untuk POST request untuk membuat Chapter baru (API Pertama)
async fn create_chapter(new_chapter: web::Json<Chapter>) -> impl Responder {
    HttpResponse::Created().json(new_chapter.into_inner())
}

// Handler untuk GET request untuk mengambil recipes (API Kedua)
async fn get_recipes() -> impl Responder {
    match read_json_file::<Vec<Recipe>>("data/recipes.json") {
        Ok(recipes) => HttpResponse::Ok().json(recipes),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to read recipes data: {}", e)),
    }
}

// Handler untuk POST request untuk membuat Recipe baru (API Kedua)
async fn create_recipe(new_recipe: web::Json<Recipe>) -> impl Responder {
    HttpResponse::Created().json(new_recipe.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // Konfigurasi CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            // Define API v1 routes untuk Chapter (API Pertama)
            .service(
                web::scope("/api/v1")
                    .route("/chapters", web::get().to(get_chapters))
                    .route("/chapters", web::post().to(create_chapter))
                    .route("/recipes", web::get().to(get_recipes))
                    .route("/recipes", web::post().to(create_recipe))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
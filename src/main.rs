use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use handlers::{chapter_handlers, recipe_handlers};

mod models;
mod handlers;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(
                web::scope("/api/v1")
                    .route("/chapters", web::get().to(chapter_handlers::get_chapters))
                    .route("/recipes", web::get().to(recipe_handlers::get_recipes))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
mod models;
mod routes;
mod query_params;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::{PgPool, PgPoolOptions};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::*;
use crate::models::*;

#[derive(Debug, Clone)]
pub struct AppData {
    pub pool: PgPool,
}

#[derive(OpenApi)]
#[openapi(paths(home, get_translations), components(schemas(TranslationInfo)))]
struct ApiDoc;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new().connect(db_url.as_str()).await.unwrap();
    let app_data = AppData { pool };
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .app_data(web::Data::new(app_data.clone()))
            .service(routes::home)
            .service(routes::get_verses)
            .service(routes::get_abbreviations)
            .service(routes::get_translations)
            .service(routes::get_books)
            .service(routes::get_translation_books)
            .service(routes::get_translation_info)
            .service(routes::get_chaptercount)
            .service(routes::get_random_verse)
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                     .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(web::redirect("/docs", "/docs/"))
    })
    .bind(("127.0.0.1", 7000))?;
    return server.run().await;
}

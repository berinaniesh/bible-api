mod models;
mod routes;

#[cfg(test)]
mod tests;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::{PgPool, PgPoolOptions};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::models::*;
use crate::routes::*;

#[derive(Debug, Clone)]
pub struct AppData {
    pub pool: PgPool,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        home,
        get_translations,
        get_verses,
        get_random_verse,
        get_abbreviations,
        get_translation_info,
        get_translation_books,
        get_chaptercount,
        search,
    ),
    components(schemas(Hello, TranslationInfo, Verse, Book, Count, SearchParameters))
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let port_string = dotenvy::var("PORT").unwrap_or(String::from("7000"));
    let port: u16 = port_string.parse().unwrap_or(7000);
    let pool = PgPoolOptions::new().connect(db_url.as_str()).await.unwrap();
    let app_data = AppData { pool };
    std::env::set_var("RUST_LOG", "warn");
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
            .service(routes::search)
            .service(
                SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(web::redirect("/docs", "/docs/"))
    })
    .bind(("127.0.0.1", port))?;
    return server.run().await;
}

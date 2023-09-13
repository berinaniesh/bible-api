use actix_web::{web, get, App, HttpServer, HttpResponse};
use actix_web::middleware::Logger;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Clone)]
struct AppData {
    verses: Vec<Verse>,
}

#[derive(Debug, Clone, Serialize)]
struct Verse {
    translation: String,
    book: String,
    abbreviation: String,
    regional_name: String,
    chapter_number: i32,
    verse_number: i32,
    verse: String,
}

#[get("/")]
async fn home() -> HttpResponse {
    HttpResponse::Ok().json(json!({"About": "REST API to serve bible verses", "TranslationsAvailable": ["TOVBSI", "KJV"]}))
}


#[get("/verse")]
async fn get_verse(app_data: web::Data<AppData>) -> HttpResponse {
    HttpResponse::Ok().json(&app_data.verses)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new().
        connect(db_url.as_str())
        .await
        .unwrap();
    let verses = sqlx::query_as!(Verse,
        r#"select t.name as translation,
        b.name as book, b.abbreviation as abbreviation,
        b.regional_name as regional_name,
        c.chapter_number as chapter_number, verse_number, verse
        from "Verse" v join "Chapter" c on v.chapter_id=c.id 
        join "Book" b on b.id=c.book_id 
        join "Translation" t on t.id=b.translation_id"#
        ).fetch_all(&pool).await.unwrap();
    let app_data = AppData { verses };
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_data.clone()))
            .service(home)
    })
    .bind(("127.0.0.1", 7000))?;
    return server.run().await;
}

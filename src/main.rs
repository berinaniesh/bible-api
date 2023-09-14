use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::FromRow;
use actix_cors::Cors;

#[derive(Debug, Clone)]
struct AppData {
    pool: PgPool,
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct Verse {
    translation: String,
    book: String,
    regional_name: String,
    chapter_number: i32,
    verse_number: i32,
    verse: String,
}

#[derive(Debug, Deserialize)]
struct Filter {
    translation: Option<String>,
    book: Option<String>,
    abbreviation: Option<String>,
    chapter: Option<i32>,
    startchapter: Option<i32>,
    endchapter: Option<i32>,
    verse: Option<i32>,
    startverse: Option<i32>,
    endverse: Option<i32>,
}

impl Filter {
    fn sanitize(&mut self) {
        if self.translation.is_some() {
            let t = self.translation.clone().unwrap();
            if t.contains("'") {
                self.translation = None;
            }
        }
        if self.book.is_some() {
            let b = self.book.clone().unwrap();
            if b.contains("'") {
                self.book = None;
            }
        }
        if self.abbreviation.is_some() {
            let a = self.abbreviation.clone().unwrap();
            if a.contains("'") {
                self.abbreviation = None;
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct TranslationName {
    name: String,
}

#[derive(Debug, Serialize)]
struct BookName {
    name: String,
}

#[derive(Debug, Serialize, FromRow)]
struct TranslationInfo {
    name: String,
    language: String,
    full_name: Option<String>,
    year: Option<String>,
    license: Option<String>,
    description: Option<String>,
}

#[allow(unused_assignments)]
fn get_query(qp: web::Query<Filter>) -> String {
    let mut is_first = true;
    let mut base_query = String::from(
        r#"select t.name as translation,
        b.name as book, b.regional_name as regional_name,
        c.chapter_number as chapter_number, verse_number, verse
        from "Verse" v join "Chapter" c on v.chapter_id=c.id 
        join "Book" b on b.id=c.book_id 
        join "Translation" t on t.id=b.translation_id"#,
    );
    if qp.translation.is_some() {
        let translation_name = qp.translation.clone().unwrap().to_uppercase();
        if is_first {
            base_query += "\n where "
        } else {
            base_query += "\n and "
        }
        let q = format!("t.name='{translation_name}'");
        base_query += q.as_str();
        is_first = false;
    }
    if qp.book.is_some() {
        let book_name = qp.book.clone().unwrap();
        if is_first {
            base_query += "\n where "
        } else {
            base_query += "\n and "
        }
        let q = format!("b.name='{book_name}'");
        base_query += q.as_str();
        is_first = false;
    }
    if qp.abbreviation.is_some() {
        let abbreviation = qp.abbreviation.clone().unwrap().to_uppercase();
        if is_first {
            base_query += "\n where "
        } else {
            base_query += "\n and "
        }
        let q = format!("b.abbreviation='{abbreviation}'");
        base_query += q.as_str();
        is_first = false;
    }
    if qp.chapter.is_some() {
        let chapter = qp.chapter.unwrap();
        let q = format!("\n and c.chapter_number='{chapter}'");
        base_query += q.as_str();
    }
    if qp.startchapter.is_some() {
        let startchapter = qp.startchapter.unwrap();
        let q = format!("\n and c.chapter_number>='{startchapter}'");
        base_query += q.as_str();
    }
    if qp.endchapter.is_some() {
        let endchapter = qp.endchapter.unwrap();
        let q = format!("\n and c.chapter_number<='{endchapter}'");
        base_query += q.as_str();
    }
    if qp.verse.is_some() {
        let verse = qp.verse.unwrap();
        let q = format!("\n and v.verse_number='{verse}'");
        base_query += q.as_str();
    }
    if qp.startverse.is_some() {
        let startverse = qp.startverse.unwrap();
        let q = format!("\n and v.verse_number>='{startverse}'");
        base_query += q.as_str();
    }
    if qp.endverse.is_some() {
        let endverse = qp.endverse.unwrap();
        let q = format!("\n and v.verse_number<='{endverse}'");
        base_query += q.as_str();
    }
    let order_query = " order by t.id,b.id,c.chapter_number,v.verse_number";
    base_query += order_query;
    return base_query;
}

#[get("/")]
async fn home(app_data: web::Data<AppData>) -> HttpResponse {
    let t = sqlx::query_as!(TranslationName, r#"SELECT name from "Translation""#)
        .fetch_all(&app_data.pool)
        .await
        .unwrap();
    let mut translations: Vec<String> = Vec::new();
    for i in t.iter() {
        translations.push(i.name.clone());
    }

    HttpResponse::Ok().json(
        json!({
            "About": "REST API to serve bible verses",
            "Repository": "https://github.com/berinaniesh/bible-api",
            "TranslationsAvailable": translations,
            "InfoAboutTranslations": "/translations",
            "VersesEndpoint": "/verses",
            "ParametersAvailable": ["translation", "book", "abbreviation", "chapter", "startchapter", "endchapter", "verse", "startverse", "endverse"],
            "Example": "/verses?translation=tovbsi&book=1+Samuel&abbreviation=1SA&chapter=1&verse=10",
            "abbreviations": "/abbreviations"
        }),
    )
}

#[get("/translations")]
async fn get_translations(app_data: web::Data<AppData>) -> HttpResponse {
    let q = sqlx::query_as!(TranslationInfo, r#"SELECT name, l.lname as language, full_name, year, license, description from "Translation" join (select id, name as lname from "Language") l on l.id=language_id"#).fetch_all(&app_data.pool).await.unwrap();
    return HttpResponse::Ok().json(q);
}

#[get("/books")]
async fn get_books(app_data: web::Data<AppData>) -> HttpResponse {
    let q = sqlx::query_as!(BookName, r#"SELECT name FROM "Book""#)
        .fetch_all(&app_data.pool)
        .await
        .unwrap();
    let mut v = Vec::new();
    for i in q.iter() {
        v.push(i.name.clone());
    }
    return HttpResponse::Ok().json(v);
}

#[get("/abbreviations")]
async fn get_abbreviations(app_data: web::Data<AppData>) -> HttpResponse {
    let q = sqlx::query!(r#"SELECT abbreviation from "Book""#)
        .fetch_all(&app_data.pool)
        .await
        .unwrap();
    let mut v = Vec::new();
    for i in q {
        v.push(i.abbreviation.clone());
    }
    return HttpResponse::Ok().json(v);
}

#[get("/verses")]
async fn get_verses(app_data: web::Data<AppData>, mut qp: web::Query<Filter>) -> HttpResponse {
    qp.sanitize();
    if qp.book.is_none() && qp.abbreviation.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Either one of book or abbreviation parameters is required"
        }));
    }
    let query = get_query(qp);
    let verses_result = sqlx::query_as::<_, Verse>(query.as_str())
        .fetch_all(&app_data.pool)
        .await;

    if verses_result.is_err() {
        return HttpResponse::InternalServerError()
            .json(json!({"message": "Something wrong with the query"}));
    }
    let verses = verses_result.unwrap();
    HttpResponse::Ok().json(&verses)
}

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
            .service(home)
            .service(get_verses)
            .service(get_abbreviations)
            .service(get_translations)
            .service(get_books)
    })
    .bind(("127.0.0.1", 7000))?;
    return server.run().await;
}

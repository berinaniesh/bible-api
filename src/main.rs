use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{FromRow, Postgres, QueryBuilder};
use actix_cors::Cors;

#[derive(Debug, Clone)]
struct AppData {
    pool: PgPool,
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct Verse {
    translation: String,
    book: String,
    book_name: String,
    chapter: i32,
    verse_number: i32,
    verse: String,
}

#[derive(Debug, Deserialize)]
struct VerseFilter {
    translation: Option<String>,
    tr: Option<String>,
    book: Option<String>,
    b: Option<String>,
    abbreviation: Option<String>,
    ab: Option<String>,
    chapter: Option<i32>,
    ch: Option<i32>,
    startchapter: Option<i32>,
    sch: Option<i32>,
    endchapter: Option<i32>,
    ech: Option<i32>,
    verse: Option<i32>,
    v: Option<i32>,
    startverse: Option<i32>,
    sv: Option<i32>,
    endverse: Option<i32>,
    ev: Option<i32>,
}

#[derive(Debug, Serialize)]
struct TranslationName {
    name: String,
}

#[derive(Debug, Deserialize)]
struct TranslationName2 {
    translation: Option<String>,
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
async fn query_verses(qp: web::Query<VerseFilter>, app_data: web::Data<AppData>) -> Vec<Verse> {
    let mut is_first = true;
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(r#"select t.name as translation, b.name as book, tt.name as book_name, c.chapter_number as chapter, v.verse_number as verse_number, verse from "VerseText" vv join "Translation" t on vv.translation_id=t.id join "Verse" v on v.id=vv.verse_id join "Chapter" c on v.chapter_id=c.id join "Book" b on c.book_id=b.id join "TranslationBookName" tt on (t.id=tt.translation_id and b.id=tt.book_id)"#);

    if let Some(x) = &qp.abbreviation {
        query_builder.push(" where b.abbreviation=");
        is_first = false;
        query_builder.push_bind(x.to_uppercase());
    }    
    if let Some(x) = &qp.ab {
        if is_first {
            query_builder.push(" where b.abbreviation=");
            is_first = false;
        } else {
            query_builder.push(" and b.abbreviation=");
        }
        query_builder.push_bind(x.to_uppercase());
    }
    if let Some(x) = &qp.book {
        if is_first {
            query_builder.push(" where b.name=");
            is_first = false;
        } else {
            query_builder.push(" and b.name=");
        }
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.b {
        if is_first {
            query_builder.push(" where b.name=");
            is_first = false;
        } else {
            query_builder.push(" and b.name=");
        }
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.ch {
        query_builder.push(" and c.chapter_number=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.chapter {
        query_builder.push(" and c.chapter_number=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.sch {
        query_builder.push(" and c.chapter_number>=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.startchapter {
        query_builder.push(" and c.chapter_number>=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.ech {
        query_builder.push(" and c.chapter_number<=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.endchapter {
        query_builder.push(" and c.chapter_number<=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.v {
        query_builder.push(" and v.verse_number=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.verse {
        query_builder.push(" and v.verse_number=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.sv {
        query_builder.push(" and v.verse_number>=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.startverse {
        query_builder.push(" and v.verse_number>=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.ev {
        query_builder.push(" and v.verse_number<=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.endverse {
        query_builder.push(" and v.verse_number<=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.tr {
        query_builder.push(" and t.name=");
        query_builder.push_bind(x.to_uppercase());
    }
    if let Some(x) = &qp.translation {
        query_builder.push(" and t.name=");
        query_builder.push_bind(x.to_uppercase());
    }
    query_builder.push(" order by vv.id");
    let query = query_builder.build_query_as::<Verse>();
    let verses = query.fetch_all(&app_data.pool).await.unwrap();
    return verses;
}

#[get("/")]
async fn home(app_data: web::Data<AppData>) -> HttpResponse {
    let t = sqlx::query_as!(TranslationName, r#"SELECT name from "Translation" order by id"#)
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
            "Endpoints": ["/translations", "/verses", "/abbreviations", "/books"],
            "ParametersForVerses": ["translation or tr", "book or b", "abbreviation or ab", "chapter or ch", "startchapter or sch", "endchapter or ech", "verse or v", "startverse or sv", "endverse or ev"],
            "ParametersForBooks": ["translation"],
            "Examples": ["/verses?translation=tovbsi&book=1+Samuel&abbreviation=1SA&chapter=1&verse=10", "/verses?tr=kjv&ab=jhn&ch=1&v=1"]
        }),
    )
}

#[get("/translations")]
async fn get_translations(app_data: web::Data<AppData>) -> HttpResponse {
    let q = sqlx::query_as!(TranslationInfo, r#"SELECT name, l.lname as language, full_name, year, license, description from "Translation" join (select id, name as lname from "Language") l on l.id=language_id"#).fetch_all(&app_data.pool).await.unwrap();
    return HttpResponse::Ok().json(q);
}

#[get("/books")]
async fn get_books(qp: web::Query<TranslationName2>, app_data: web::Data<AppData>) -> HttpResponse {
    let mut ot = Vec::new();
    let mut nt = Vec::new();
    let q;
    if qp.translation.is_some() {
        let t = qp.translation.clone().unwrap();
        q = sqlx::query_as!(BookName, r#"SELECT name from "TranslationBookName" where translation_id=(select id from "Translation" where name=$1)"#, t.to_uppercase()).fetch_all(&app_data.pool).await.unwrap();
    } else {
        q = sqlx::query_as!(BookName, r#"SELECT name FROM "Book" order by id"#)
        .fetch_all(&app_data.pool)
        .await
        .unwrap();
    }
    if q.len() == 66 { 
        for i in 0..39 {
            ot.push(q[i].name.clone());
        }
        for i in 39..66 {
            nt.push(q[i].name.clone());
        }
    } else {
        return HttpResponse::BadRequest().json(
            json!({
                "message": "Not all books were fetched, check if the translation name is correct"
            })
        );
    }
    return HttpResponse::Ok().json(
        json!({
            "Old Testament": ot,
            "New Testament": nt,
        })
    );
}

#[get("/abbreviations")]
async fn get_abbreviations(app_data: web::Data<AppData>) -> HttpResponse {
    let q = sqlx::query!(r#"SELECT abbreviation from "Book" order by id"#)
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
async fn get_verses(app_data: web::Data<AppData>, qp: web::Query<VerseFilter>) -> HttpResponse {
    if qp.book.is_none() && qp.abbreviation.is_none() && qp.ab.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Either one of book or abbreviation parameters is required"
        }));
    }
    let query = query_verses(qp, app_data).await;
    return HttpResponse::Ok().json(query);
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

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::FromRow;

#[derive(Debug, Clone)]
struct AppData {
    pool: PgPool,
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct Verse {
    translation: String,
    book: String,
    abbreviation: String,
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
    startchapter: Option<i32>,
    endchapter: Option<i32>,
    startverse: Option<i32>,
    endverse: Option<i32>
}

#[derive(Serialize)]
struct Translation {
    name: String,
}

#[allow(unused_assignments)]
fn get_query(qp: web::Query<Filter>) -> String {
    let mut is_first = true;
    let mut base_query = String::from(
        r#"select t.name as translation,
        b.name as book, b.abbreviation as abbreviation,
        b.regional_name as regional_name,
        c.chapter_number as chapter_number, verse_number, verse
        from "Verse" v join "Chapter" c on v.chapter_id=c.id 
        join "Book" b on b.id=c.book_id 
        join "Translation" t on t.id=b.translation_id"#
        );
    if qp.translation.is_some() {
        let translation_name = qp.translation.clone().unwrap();
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
    if qp.startchapter.is_some() {
        let startchapter=qp.startchapter.unwrap();
        if is_first {
            base_query += "\n where "
        } else {
            base_query += "\n and "
        }
        let q = format!("c.chapter_number>='{startchapter}'");
        base_query += q.as_str();
        is_first = false;
    }
    if qp.endchapter.is_some() {
        let endchapter=qp.endchapter.unwrap();
        if is_first {
            base_query += "\n where "
        } else {
            base_query += "\n and "
        }
        let q = format!("c.chapter_number<='{endchapter}'");
        base_query += q.as_str();
        is_first = false;
    }
    if qp.startverse.is_some() {
        let startverse=qp.startverse.unwrap();
        if is_first {
            base_query += "\n where "
        } else {
            base_query += "\n and "
        }
        let q = format!("v.verse_number>='{startverse}'");
        base_query += q.as_str();
        is_first = false;
    }
    if qp.endverse.is_some() {
        let endverse=qp.endverse.unwrap();
        if is_first {
            base_query += "\n where "
        } else {
            base_query += "\n and "
        }
        let q = format!("v.verse_number<='{endverse}'");
        base_query += q.as_str();
        is_first = false;
    }
    let order_query = " order by t.id,b.id,c.chapter_number,v.verse_number";
    base_query += order_query;
    return base_query;
}

#[get("/")]
async fn home(app_data: web::Data<AppData>) -> HttpResponse {
    let t = sqlx::query_as!(Translation, r#"SELECT name from "Translation""#)
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
            "TranslationsAvailable": translations,
            "Endpoint": "/verses",
            "Hint": "Make sure to use query parameters to restrict the number of verses fetched",
            "Example": "/verses?translation=TOVBSI&book=1+Samuel&abbreviation=1SA&startchapter=1&endchapter=3&startverse=1&endverse=20"
        }),
    )
}

#[get("/verses")]
async fn get_verses(app_data: web::Data<AppData>, qp: web::Query<Filter>) -> HttpResponse {
    let query = get_query(qp);
    let verses_result = sqlx::query_as::<_, Verse>(
        query.as_str()
    )
    .fetch_all(&app_data.pool)
    .await;

    if verses_result.is_err() {
        return HttpResponse::InternalServerError().json(json!({"message": "Something wrong with the query"}));
    }
    let verses = verses_result.unwrap();
    HttpResponse::Ok().json(&verses)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new().connect(db_url.as_str()).await.unwrap();
    let app_data = AppData { pool };
    std::env::set_var("RUST_LOG", "warn");
    env_logger::init();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_data.clone()))
            .service(home)
            .service(get_verses)
    })
    .bind(("127.0.0.1", 7000))?;
    return server.run().await;
}

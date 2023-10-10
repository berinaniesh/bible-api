use serde_json::json;
use rand::{thread_rng, Rng};
use sqlx::{Postgres, QueryBuilder};
use actix_web::{get, web, HttpResponse};

use crate::models::*;
use crate::AppData;
use crate::query_params::*;

#[allow(unused_assignments)]
pub async fn query_verses(qp: web::Query<VerseFilter>, app_data: web::Data<AppData>) -> Vec<Verse> {
    let mut is_first = true;
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"select t.name as translation, b.name as book, tt.name as book_name, c.chapter_number as chapter, v.verse_number as verse_number, verse from "VerseText" vv join "Translation" t on vv.translation_id=t.id join "Verse" v on v.id=vv.verse_id join "Chapter" c on v.chapter_id=c.id join "Book" b on c.book_id=b.id join "TranslationBookName" tt on (t.id=tt.translation_id and b.id=tt.book_id)"#,
    );

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

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "API healthy")
    )
)]
#[get("/")]
pub async fn home(app_data: web::Data<AppData>) -> HttpResponse {
    let t = sqlx::query_as!(
        TranslationName,
        r#"SELECT name from "Translation" order by id"#
    )
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

#[utoipa::path(
    get,
    path = "/translations",
    responses(
        (status = 200, description = "List of bible translations available", body = TranslationInfo)
    )
)]
#[get("/translations")]
pub async fn get_translations(app_data: web::Data<AppData>) -> HttpResponse {
    let q = sqlx::query_as!(TranslationInfo, r#"SELECT name, l.lname as language, full_name, year, license, description from "Translation" t join (select id, name as lname from "Language") l on l.id=language_id ORDER BY t.id"#).fetch_all(&app_data.pool).await.unwrap();
    return HttpResponse::Ok().json(q);
}

#[get("/books")]
pub async fn get_books(qp: web::Query<TranslationSelector>, app_data: web::Data<AppData>) -> HttpResponse {
    let mut ot = Vec::new();
    let mut nt = Vec::new();
    let mut translation_name = String::new();
    let q;
    if qp.translation.is_some() {
        translation_name = qp.translation.clone().unwrap();
    } else if qp.tr.is_some() {
        translation_name = qp.tr.clone().unwrap();
    }
    if !translation_name.is_empty() {
        q = sqlx::query_as!(BookName, r#"SELECT name from "TranslationBookName" where translation_id=(select id from "Translation" where name=$1)"#, translation_name.to_uppercase()).fetch_all(&app_data.pool).await.unwrap();
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
        return HttpResponse::BadRequest().json(json!({
            "message": "Not all books were fetched, check if the translation name is correct"
        }));
    }
    return HttpResponse::Ok().json(json!({
        "Old Testament": ot,
        "New Testament": nt,
    }));
}

#[utoipa::path(
    get,
    path = "/abbreviations",
    responses(
        (status = 200, description = "Get a list of abbreviations supported"),
    ),
)]
#[get("/abbreviations")]
pub async fn get_abbreviations(app_data: web::Data<AppData>) -> HttpResponse {
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

#[utoipa::path(
    get,
    path = "/verses",
    params (VerseFilter),
    responses(
        (status = 200, description = "Get verses based on query parameters", body = Verse),
        (status = 400, description = "Either of book or abbreviation parameters is required")
    ),
)]
#[get("/verses")]
pub async fn get_verses(app_data: web::Data<AppData>, qp: web::Query<VerseFilter>) -> HttpResponse {
    if qp.book.is_none() && qp.b.is_none() && qp.abbreviation.is_none() && qp.ab.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Either one of book or abbreviation parameters is required"
        }));
    }
    let query = query_verses(qp, app_data).await;
    return HttpResponse::Ok().json(query);
}

#[utoipa::path(
    get,
    path = "/verses/random",
    params (TranslationSelector),
    responses(
        (status = 200, description = "Get a random verse (not filtered for good verses, beware)", body = Verse),
    ),
)]
#[get("/verses/random")]
pub async fn get_random_verse(
    app_data: web::Data<AppData>,
    parameters: web::Query<TranslationSelector>,
) -> HttpResponse {
    let r: i32 = thread_rng().gen_range(1..31102);
    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"select t.name as translation, b.name as book, bb.name as book_name, c.chapter_number as chapter, v.verse_number as verse_number, vv.verse as verse from "VerseText" vv join "Translation" t on t.id=vv.translation_id join "Verse" v on v.id=vv.verse_id join "Chapter" c on c.id=v.chapter_id join "Book" b on b.id=c.book_id join "TranslationBookName" bb on bb.book_id=b.id and vv.translation_id=bb.translation_id where vv.verse_id="#,
    );
    qb.push_bind(r);
    if parameters.translation.is_some() {
        let tr = parameters.translation.clone().unwrap().to_uppercase();
        qb.push(" and t.name=");
        qb.push_bind(tr);
    } else if parameters.tr.is_some() {
        let tr = parameters.tr.clone().unwrap().to_uppercase();
        qb.push(" and t.name=");
        qb.push_bind(tr);
    }
    let query = qb.build_query_as::<Verse>();
    let verses = query.fetch_all(&app_data.pool).await.unwrap();
    return HttpResponse::Ok().json(verses);
}

#[utoipa::path(
    get,
    path = "/chaptercount/{book}",
    responses(
        (status = 200, description = "Number of chapters in a book", body = Count),
    ),
)]
#[get("/chaptercount/{book}")]
pub async fn get_chaptercount(app_data: web::Data<AppData>, path: web::Path<String>) -> HttpResponse {
    let book = path.into_inner();
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"SELECT COUNT(*) AS count FROM "Chapter" WHERE book_id=(SELECT id FROM "Book" WHERE name="#,
    );
    query_builder.push_bind(book);
    query_builder.push(")");
    let query = query_builder.build_query_as::<Count>();
    let count = query.fetch_one(&app_data.pool).await.unwrap();
    return HttpResponse::Ok().json(count);
}

#[utoipa::path(
    get,
    path = "/{translation}/info",
    responses(
        (status = 200, description = "Get information about specific translation", body = TranslationInfo),
    ),
)]
#[get("/{translation}/info")]
pub async fn get_translation_info(
    app_data: web::Data<AppData>,
    path: web::Path<String>,
) -> HttpResponse {
    let translation = path.into_inner().to_uppercase();
    let q = sqlx::query_as!(TranslationInfo, r#"SELECT name, l.lname as language, full_name, year, license, description from "Translation" join (select id, name as lname from "Language") l on l.id=language_id WHERE name=$1"#, &translation).fetch_one(&app_data.pool).await;
    if q.is_err() {
        return HttpResponse::BadRequest().json(json!(format!(
            "The requested translation {} is not found on the server",
            &translation
        )));
    }
    return HttpResponse::Ok().json(q.unwrap());
}

#[utoipa::path(
    get,
    path = "/{translation}/books",
    responses(
        (status = 200, description = "Get list of books with respect to the translation", body = Book),
    ),
)]
#[get("/{translation}/books")]
pub async fn get_translation_books(
    app_data: web::Data<AppData>,
    path: web::Path<String>,
) -> HttpResponse {
    let translation = path.into_inner().to_uppercase();
    let q = sqlx::query_as!(Book, r#"select b.id book_id, tb.name book_name, tn.name testament from "Book" b join "TestamentName" tn on b.testament=tn.testament join "Translation" t on t.id=tn.translation_id join "TranslationBookName" tb on tb.translation_id=t.id and b.id=tb.book_id where t.name=$1 order by b.id"#, &translation).fetch_all(&app_data.pool).await.unwrap();
    if q.is_empty() {
        return HttpResponse::BadRequest().json(json!(format!(
            "The requested translation {} is not found on the server",
            &translation
        )));
    }
    return HttpResponse::Ok().json(q);
}
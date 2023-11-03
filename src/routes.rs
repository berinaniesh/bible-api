use actix_web::{get, post, web, HttpResponse};
use rand::{thread_rng, Rng};
use serde_json::json;
use sqlx::{Postgres, QueryBuilder};

use crate::models::*;
use crate::AppData;

#[allow(unused_assignments)]
pub async fn query_verses(qp: web::Query<VerseFilter>, app_data: web::Data<AppData>) -> Vec<Verse> {
    let mut is_first = true;
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"SELECT translation, book, book_name, chapter, verse_number, verse FROM fulltable"#,
    );

    if let Some(x) = &qp.abbreviation {
        query_builder.push(" WHERE abbreviation=");
        is_first = false;
        query_builder.push_bind(x.to_uppercase());
    }
    if let Some(x) = &qp.ab {
        if is_first {
            query_builder.push(" WHERE abbreviation=");
            is_first = false;
        } else {
            query_builder.push(" AND abbreviation=");
        }
        query_builder.push_bind(x.to_uppercase());
    }
    if let Some(x) = &qp.book {
        if is_first {
            query_builder.push(" WHERE book=");
            is_first = false;
        } else {
            query_builder.push(" AND book=");
        }
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.b {
        if is_first {
            query_builder.push(" WHERE book=");
            is_first = false;
        } else {
            query_builder.push(" AND book=");
        }
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.ch {
        query_builder.push(" AND chapter=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.chapter {
        query_builder.push(" AND chapter=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.sch {
        query_builder.push(" AND chapter>=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.startchapter {
        query_builder.push(" AND chapter>=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.ech {
        query_builder.push(" AND chapter<=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.endchapter {
        query_builder.push(" AND chapter<=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.v {
        query_builder.push(" AND verse_number=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.verse {
        query_builder.push(" AND verse_number=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.sv {
        query_builder.push(" AND verse_number>=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.startverse {
        query_builder.push(" AND verse_number>=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.ev {
        query_builder.push(" AND verse_number<=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.endverse {
        query_builder.push(" AND verse_number<=");
        query_builder.push_bind(x);
    }
    if let Some(x) = &qp.tr {
        query_builder.push(" AND translation=");
        query_builder.push_bind(x.to_uppercase());
    }
    if let Some(x) = &qp.translation {
        query_builder.push(" AND translation=");
        query_builder.push_bind(x.to_uppercase());
    }
    query_builder.push(" ORDER BY id");
    let query = query_builder.build_query_as::<Verse>();
    let verses = query.fetch_all(&app_data.pool).await.unwrap();
    return verses;
}

/// Hello Message
#[utoipa::path(
    get,
    tag = "Hello",
    path = "/",
    responses(
        (status = 200, description = "API healthy", body = Hello)
    )
)]
#[get("/")]
pub async fn home() -> HttpResponse {
    let hello = Hello::default();
    return HttpResponse::Ok().json(hello);
}

/// Get a list of available translations
#[utoipa::path(
    get,
    tag = "Info",
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


/// Get a list of Bible books
#[utoipa::path(
    get,
    tag = "Info",
    path = "/books",
    responses(
        (status = 200, description = "List of bible books", body = BookName)
    )
)]
#[get("/books")]
pub async fn get_books(
    qp: web::Query<TranslationSelector>,
    app_data: web::Data<AppData>,
) -> HttpResponse {
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

/// Get a list of abbreviations of books
#[utoipa::path(
    get,
    tag = "Info",
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

/// Get a list of verses by filtering with VerseFilter
#[utoipa::path(
    get,
    tag = "Verse",
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

/// Get a random verse (not filtered to get good verses)
#[utoipa::path(
    get,
    tag = "Verse",
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

/// Get the number of chapters in a book
#[utoipa::path(
    get,
    tag = "Info",
    path = "/chaptercount/{book}",
    responses(
        (status = 200, description = "Number of chapters in a book", body = Count),
    ),
)]
#[get("/chaptercount/{book}")]
pub async fn get_chaptercount(
    app_data: web::Data<AppData>,
    path: web::Path<String>,
) -> HttpResponse {
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

/// Get information about a specific translation
#[utoipa::path(
    get,
    tag = "Info",
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

/// Get a list of books with respect to the translation
/// 
/// The name of the book in the translation language, etc
#[utoipa::path(
    get,
    tag = "Info",
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
    let q = sqlx::query_as!(Book, r#"
        SELECT b.id book_id, b.abbreviation abbreviation,
        tb.name book_name, b.name book, b.testament as "testament: Testament",
        tn.name testament_name from "Book" b 
        join "TestamentName" tn on b.testament=tn.testament 
        join "Translation" t on t.id=tn.translation_id 
        join "TranslationBookName" tb 
        on tb.translation_id=t.id and b.id=tb.book_id 
        where t.name=$1 order by b.id
        "#, &translation).fetch_all(&app_data.pool).await.unwrap();
    if q.is_empty() {
        return HttpResponse::BadRequest().json(json!(format!(
            "The requested translation {} is not found on the server",
            &translation
        )));
    }
    return HttpResponse::Ok().json(q);
}

/// Get verses based on text search
#[utoipa::path(
    post,
    tag = "Verse",
    path = "/search",
    request_body = SearchParameters,
    responses(
        (status = 200, description = "Search throughout the bible", body = Verse),
    ),
)]
#[post("/search")]
pub async fn search(search_parameters: web::Json<SearchParameters>, app_data: web::Data<AppData>) -> HttpResponse {
    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(r#"
    SELECT translation, book, book_name, chapter, verse_number, verse from fulltable where verse "#);
    let match_case = search_parameters.match_case.unwrap_or(false);
    if match_case {
        qb.push("like(");
    } else {
        qb.push("ilike(");
    }
    let actual_search_string = format!("%{}%", &search_parameters.search_text);
    qb.push_bind(actual_search_string);
    qb.push(")");
    if search_parameters.translation.is_some() {
        qb.push(" and translation=");
        qb.push_bind(search_parameters.translation.clone().unwrap().to_uppercase());
    }
    let query = qb.build_query_as::<Verse>();
    let verses = query.fetch_all(&app_data.pool).await.unwrap();
    return HttpResponse::Ok().json(verses);
}

/// Get the next chapter / book to go to
///
/// The frontend needs to know what page to go to once
/// a user finishes reading one chapter and since the frontend
/// doesn't have access to the database and it needs a few calls to
/// the API to figure it out, it'd be nice to have the API give 
/// the information directly. 
#[utoipa::path(
    post,
    tag = "Frontend Helper",
    path = "/next",
    request_body = CurrentPage,
    responses(
        (status = 200, description = "Returns info about the next page to navigate to", body = NextPage),
        (status = 400, description = "Atleast one argument of book or abbreviation is required",),
    ),
)]
#[post("/next")]
pub async fn get_next_page(current_page: web::Json<CurrentPage>, app_data: web::Data<AppData>) -> HttpResponse {
    if current_page.book.is_none() && current_page.abbreviation.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Either one of book or abbreviation is required"
        }))
    }
    let mut is_revelation = false;
    if current_page.book.is_some() {
        let book = current_page.book.clone().unwrap();
        if book == "Revelation" {
            is_revelation = true;
        }
    }
    if current_page.abbreviation.is_some() {
        let abbreviation = current_page.abbreviation.clone().unwrap().to_uppercase();
        if abbreviation == "REV" {
            is_revelation = true;
        }
    }
    if is_revelation && current_page.chapter == 22 {
        let next_page = NextPage{book: "Genesis".to_string(), abbreviation: "GEN".to_string(), chapter: 1, bible_ended: true};
        return HttpResponse::Ok().json(next_page);
    }
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"SELECT COUNT(*) AS count FROM "Chapter" WHERE book_id=(SELECT id FROM "Book" WHERE"#,
    );
    if current_page.book.is_some() {
        let book = current_page.book.clone().unwrap();
        query_builder.push(" name=");
        query_builder.push_bind(book);
        query_builder.push(")");
    } else {
        let abbreviation = current_page.abbreviation.clone().unwrap();
        query_builder.push(" abbreviation=");
        query_builder.push_bind(abbreviation);
        query_builder.push(")");
    }
    let query = query_builder.build_query_as::<Count>();
    let current_book_chapter_count = query.fetch_one(&app_data.pool).await.unwrap().count;
    if current_page.chapter < current_book_chapter_count {
//        let next_page = NextPage {}
    }
    return HttpResponse::Ok().json("hello");
}

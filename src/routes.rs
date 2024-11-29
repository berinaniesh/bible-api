use actix_web::{get, post, web, HttpResponse};
use rand::{thread_rng, Rng};
use serde_json::json;
use sqlx::{Postgres, QueryBuilder};

use crate::error::AppError;
use crate::models::*;
use crate::AppData;

#[allow(unused_assignments)]
pub async fn query_verses(qp: web::Query<VerseFilter>, app_data: web::Data<AppData>) -> Vec<Verse> {
    let mut is_first = true;
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"SELECT translation, book, abbreviation, book_name, chapter, verse_number, verse FROM fulltable"#,
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
    params (TranslationSelector),
    responses(
        (status = 200, description = "List of bible books")
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
        q = sqlx::query_as!(BookName, r#"SELECT name from "TranslationBookName" where translation_id=(select id from "Translation" where name=$1) order by id"#, translation_name.to_uppercase()).fetch_all(&app_data.pool).await.unwrap();
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
        r#"select t.name as translation, b.name as book, b.abbreviation as abbreviation, bb.name as book_name, c.chapter_number as chapter, v.verse_number as verse_number, vv.verse as verse from "VerseText" vv join "Translation" t on t.id=vv.translation_id join "Verse" v on v.id=vv.verse_id join "Chapter"
        c on c.id=v.chapter_id join "Book"
        b on b.id=c.book_id join "TranslationBookName"
        bb on bb.book_id=b.id and vv.translation_id=bb.translation_id
        where vv.verse_id="#,
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

///Get the number of chapters in all books
///
///This is hardcoded and the endpoint does not use the database
#[utoipa::path(
    get,
    tag = "Info",
    path = "/chaptercount",
    responses(
        (status = 200, description = "Number of chapters in all books", body = BooksChapterCount)
        )
    )
]
#[get("/chaptercount")]
pub async fn get_chaptercount() -> HttpResponse {
    let ans = BooksChapterCount::default();
    return HttpResponse::Ok().json(ans);
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
pub async fn get_chaptercount_book(
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
) -> Result<HttpResponse, AppError> {
    let translation = path.into_inner().to_uppercase();
    let q = sqlx::query_as!(
        TranslationInfo,
        r#"
        SELECT name, l.lname AS language,
        full_name, year, license, description
        FROM "Translation" JOIN
        (SELECT id, name AS lname FROM "Language") l
        ON l.id=language_id WHERE name=$1"#,
        &translation
    )
    .fetch_one(&app_data.pool)
    .await?;
    return Ok(HttpResponse::Ok().json(q));
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
    let q = sqlx::query_as!(
        Book,
        r#"
        SELECT b.id book_id, b.abbreviation abbreviation,
        tb.name book_name, b.name book, b.testament as "testament: Testament",
        tn.name testament_name from "Book" b 
        join "TestamentName" tn on b.testament=tn.testament 
        join "Translation" t on t.id=tn.translation_id 
        join "TranslationBookName" tb 
        on tb.translation_id=t.id and b.id=tb.book_id 
        where t.name=$1 order by b.id
        "#,
        &translation
    )
    .fetch_all(&app_data.pool)
    .await
    .unwrap();
    if q.is_empty() {
        return HttpResponse::BadRequest().json(json!(format!(
            "The requested translation {} is not found on the server",
            &translation
        )));
    }
    return HttpResponse::Ok().json(q);
}

/// Get verses based on text search
///
/// If the length of the search text is less than 3, an empty array is returned. (Not errored as the frontend does not have good error handling).
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
pub async fn search(
    search_parameters: web::Json<SearchParameters>,
    app_data: web::Data<AppData>,
) -> HttpResponse {
    if search_parameters.search_text.len() < 3 {
        return HttpResponse::Ok().json(Vec::<String>::new());
    }
    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
    SELECT translation, book, abbreviation, book_name, chapter, verse_number, verse from fulltable where verse "#,
    );
    let match_case = search_parameters.match_case.unwrap_or(false);
    let whole_words = search_parameters.whole_words.unwrap_or(false);
    if whole_words {
        if match_case {
            qb.push("~ ");
        } else {
            qb.push("~* ");
        }
        let actual_search_string = format!(r#"\m{}\M"#, &search_parameters.search_text.trim());
        qb.push_bind(actual_search_string);
    } else {
        if match_case {
            qb.push("like ");
        } else {
            qb.push("ilike ");
        }
        let actual_search_string = format!("%{}%", &search_parameters.search_text.trim());
        qb.push_bind(actual_search_string);
    }
    qb.push(" and translation=");
    qb.push_bind(search_parameters.translation.to_uppercase());
    if let Some(book) = &search_parameters.book {
        qb.push(" and book=");
        qb.push_bind(book);
    }
    if let Some(abbreviaton) = &search_parameters.abbreviation {
        qb.push(" and abbreviation=");
        qb.push_bind(abbreviaton.to_uppercase());
    }
    let query = qb.build_query_as::<Verse>();
    let verses = query.fetch_all(&app_data.pool).await.unwrap();
    return HttpResponse::Ok().json(verses);
}

/// Get the previous and next chapter / book to go to
///
/// The frontend needs to know what page lies before and
/// after a specific chapter. So, instead of making multiple
/// API calls, the information is sent in a separate endpoint
#[utoipa::path(
    post,
    tag = "Frontend Helper",
    path = "/nav",
    request_body = PageIn,
    responses(
        (status = 200, description = "Returns info about the previous and next pages to navigate to", body = PrevNext),
        (status = 400, description = "Atleast one argument of book or abbreviation is required",),
    ),
)]
#[post("/nav")]
pub async fn get_next_page(
    current_page: web::Json<PageIn>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
    if current_page.book.is_none() && current_page.abbreviation.is_none() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "message": "Either one of book or abbreviation is required"
        })));
    }
    let previous: Option<PageOut>;
    let next: Option<PageOut>;
    let book_id;
    if let Some(ref x) = current_page.book {
        book_id = sqlx::query!(
            r#"
            SELECT id FROM "Book" WHERE name=$1
            "#,
            x
        )
        .fetch_one(&app_data.pool)
        .await?
        .id;
    } else {
        let abbreviation = current_page.abbreviation.clone().unwrap().to_uppercase();
        book_id = sqlx::query!(
            r#"
            SELECT id FROM "Book" WHERE abbreviation=$1
            "#,
            abbreviation
        )
        .fetch_one(&app_data.pool)
        .await?
        .id;
    }

    if current_page.chapter == 0 {
        if book_id == 1 {
            previous = None
        } else {
            let p = sqlx::query!(
                r#"
                SELECT name, abbreviation FROM "Book" where id=$1
                "#,
                book_id - 1
            )
            .fetch_one(&app_data.pool)
            .await?;
            previous = Some(PageOut {
                book: p.name,
                abbreviation: p.abbreviation,
                chapter: 0,
            });
        }
        if book_id == 66 {
            next = None
        } else {
            let n = sqlx::query!(
                r#"
                SELECT name, abbreviation FROM "Book" WHERE id=$1
                "#,
                book_id + 1
            )
            .fetch_one(&app_data.pool)
            .await?;
            next = Some(PageOut {
                book: n.name,
                abbreviation: n.abbreviation,
                chapter: 0,
            });
        }
        let prev_next = PrevNext { previous, next };
        return Ok(HttpResponse::Ok().json(prev_next));
    }

    if book_id == 1 && current_page.chapter == 1 {
        previous = None;
    } else {
        if current_page.chapter == 1 {
            let previous_chapter_count = sqlx::query!(
                r#"
                SELECT COUNT(*) AS count FROM "Chapter" WHERE book_id=$1
                "#,
                book_id - 1
            )
            .fetch_one(&app_data.pool)
            .await?
            .count
            .unwrap();
            let previous_book = sqlx::query!(
                r#"
                SELECT name, abbreviation FROM "Book" WHERE id=$1
                "#,
                book_id - 1
            )
            .fetch_one(&app_data.pool)
            .await?;
            previous = Some(PageOut {
                book: previous_book.name,
                abbreviation: previous_book.abbreviation,
                chapter: previous_chapter_count,
            });
        } else {
            let prev = sqlx::query!(
                r#"
                SELECT name, abbreviation FROM "Book" WHERE id=$1
                "#,
                book_id
            )
            .fetch_one(&app_data.pool)
            .await?;
            previous = Some(PageOut {
                book: prev.name,
                abbreviation: prev.abbreviation,
                chapter: current_page.chapter - 1,
            });
        }
    }

    if book_id == 66 && current_page.chapter == 22 {
        next = None;
    } else {
        let current_book_length = sqlx::query!(
            r#"
            SELECT COUNT(*) FROM "Chapter" WHERE book_id=$1
            "#,
            book_id
        )
        .fetch_one(&app_data.pool)
        .await?
        .count
        .unwrap();
        if current_page.chapter == current_book_length {
            let next_book = sqlx::query!(
                r#"
                SELECT name, abbreviation FROM "Book" WHERE id=$1
                "#,
                book_id + 1
            )
            .fetch_one(&app_data.pool)
            .await?;
            next = Some(PageOut {
                book: next_book.name,
                abbreviation: next_book.abbreviation,
                chapter: 1,
            })
        } else {
            let bo = sqlx::query!(
                r#"
                SELECT name, abbreviation FROM "Book" WHERE id=$1
                "#,
                book_id
            )
            .fetch_one(&app_data.pool)
            .await?;
            next = Some(PageOut {
                book: bo.name,
                abbreviation: bo.abbreviation,
                chapter: current_page.chapter + 1,
            });
        }
    }
    let prev_next = PrevNext { previous, next };

    return Ok(HttpResponse::Ok().json(prev_next));
}

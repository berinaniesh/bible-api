#[derive(Debug)]
struct V {
    translation: String,
    book: String,
    abbreviation: String,
    regional_name: String,
    chapter_number: i32,
    verse_number: i32,
    verse: String,
}

#[tokio::main]
async fn main() {
    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new().
        connect(db_url.as_str())
        .await
        .unwrap();
    let a = sqlx::query_as!(V,
        r#"select t.name as translation,
        b.name as book, b.abbreviation as abbreviation,
        b.regional_name as regional_name,
        c.chapter_number as chapter_number, verse_number, verse
        from "Verse" v join "Chapter" c on v.chapter_id=c.id 
        join "Book" b on b.id=c.book_id 
        join "Translation" t on t.id=b.translation_id"#
        ).fetch_all(&pool).await.unwrap();
    dbg!(&a[0]);
}

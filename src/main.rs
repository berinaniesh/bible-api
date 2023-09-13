
#[derive(Debug)]
struct V {
    bname: String,
    chapter_n: i32,
    verse_number: i32,
    verse: String,
}

#[tokio::main]
async fn main() {
    let pool = sqlx::postgres::PgPoolOptions::new().
        connect("postgresql://berinaniesh:nanbenda@localhost:5432/bible")
        .await
        .unwrap();
    let a = sqlx::query_as!(V, r#"select b.name as bname, c.chapter_number as chapter_n, verse_number, verse from "Verse" v join "Chapter" c on v.chapter_id=c.id join "Book" b on b.id=c.book_id join "Translation" t on t.id=b.translation_id"#).fetch_all(&pool).await.unwrap();
    dbg!(a);
}

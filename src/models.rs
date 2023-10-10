use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Verse {
    pub translation: String,
    pub book: String,
    pub book_name: String,
    pub chapter: i32,
    pub verse_number: i32,
    pub verse: String,
}


#[derive(Debug, Serialize, ToSchema)]
pub struct TranslationInfo {
    pub name: String,
    pub language: String,
    pub full_name: Option<String>,
    pub year: Option<String>,
    pub license: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct TranslationName {
    pub name: String,
}

#[derive(Debug)]
pub struct BookName {
    pub name: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TranslationBook {
    pub book_number: i32,
    pub book_name: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Count {
    pub count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Book {
    pub book_id: i32,
    pub book_name: String,
    pub testament: String,
}
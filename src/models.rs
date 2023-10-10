use serde::Serialize;
use sqlx::FromRow;
use std::env;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct Hello {
    pub greeting: String,
    pub name: String,
    pub about: String,
    pub docs: String,
    pub repository: String,
    pub author: String,
}

impl Hello {
    pub fn default() -> Self {
        let greeting = String::from("Hello there");
        let name = String::from("Bible API");
        let about = String::from("REST API to serve bible verses");
        let docs = String::from("/docs");
        let repository = String::from(env!("CARGO_PKG_REPOSITORY"));
        let author = String::from(env!("CARGO_PKG_AUTHORS"));

        return Hello {
            greeting: greeting,
            name: name,
            about: about,
            docs: docs,
            repository: repository,
            author: author,
        };
    }
}

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
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

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct Count {
    pub count: i64,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct Book {
    pub book_id: i32,
    pub book_name: String,
    pub testament: String,
}

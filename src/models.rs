use crate::constants::{ABBREVIATIONS, BOOKS, CHAPTERCOUNT, TOTAL, VERSECOUNT};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::env;
use std::fmt;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Type)]
#[sqlx(type_name = "testament")]
pub enum Testament {
    OldTestament,
    NewTestament,
}

impl fmt::Display for Testament {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::OldTestament => {
                //fmt::Debug::fmt("Old", f)
                write!(f, "Old Testament")
            }
            Self::NewTestament => {
                //fmt::Debug::fmt("New", f)
                write!(f, "New Testament")
            }
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Hello {
    pub greeting: String,
    pub name: String,
    pub about: String,
    pub docs: String,
    pub repository: String,
    pub database: String,
    pub text_encoding: String,
    pub author: String,
}

impl Hello {
    pub fn default() -> Self {
        let greeting = String::from("Hello there");
        let name = String::from("Bible API");
        let about = String::from("REST API to serve bible verses");
        let docs = String::from("/docs");
        let repository = String::from(env!("CARGO_PKG_REPOSITORY"));
        let database = String::from("Postgresql");
        let text_encoding = String::from("UTF-8");
        let author = String::from(env!("CARGO_PKG_AUTHORS"));

        return Hello {
            greeting: greeting,
            name: name,
            about: about,
            docs: docs,
            repository: repository,
            database: database,
            text_encoding: text_encoding,
            author: author,
        };
    }
}

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
pub struct Verse {
    pub translation: String,
    pub book: String,
    pub abbreviation: String,
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
    pub book: String,
    pub book_name: String,
    pub abbreviation: String,
    #[schema(value_type = String)]
    pub testament: Testament,
    pub testament_name: String,
}

// From user

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct VerseFilter {
    pub translation: Option<String>,
    pub tr: Option<String>,
    pub book: Option<String>,
    pub b: Option<String>,
    pub abbreviation: Option<String>,
    pub ab: Option<String>,
    pub chapter: Option<i32>,
    pub ch: Option<i32>,
    pub startchapter: Option<i32>,
    pub sch: Option<i32>,
    pub endchapter: Option<i32>,
    pub ech: Option<i32>,
    pub verse: Option<i32>,
    pub v: Option<i32>,
    pub startverse: Option<i32>,
    pub sv: Option<i32>,
    pub endverse: Option<i32>,
    pub ev: Option<i32>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct TranslationSelector {
    pub translation: Option<String>,
    pub tr: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SearchParameters {
    pub search_text: String,
    pub match_case: Option<bool>,
    pub whole_words: Option<bool>,
    pub translation: String,
    pub book: Option<String>,
    pub abbreviation: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PageIn {
    pub book: Option<String>,
    pub abbreviation: Option<String>,
    pub chapter: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PageOut {
    pub book: String,
    pub abbreviation: String,
    pub chapter: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PrevNext {
    pub previous: Option<PageOut>,
    pub next: Option<PageOut>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BooksChapterCount {
    pub book: String,
    pub abbreviation: String,
    pub chapters: u8,
    pub verses: u16,
}

impl BooksChapterCount {
    pub fn default() -> Vec<BooksChapterCount> {
        let mut ans: Vec<BooksChapterCount> = Vec::with_capacity(TOTAL as usize);
        for i in 0..TOTAL as usize {
            let temp = BooksChapterCount {
                book: String::from(BOOKS[i]),
                abbreviation: String::from(ABBREVIATIONS[i]),
                chapters: CHAPTERCOUNT[i],
                verses: VERSECOUNT[i],
            };
            ans.push(temp)
        }
        return ans;
    }
}

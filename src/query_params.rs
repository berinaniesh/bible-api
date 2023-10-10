use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct TranslationSelector {
    pub translation: Option<String>,
    pub tr: Option<String>,
}
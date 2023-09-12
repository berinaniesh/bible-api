enum Language {
    English,
    Tamil,
}

enum Translation {
    KJV,
    TOVBSI,
}

impl Translation {
    fn full_form(&self) -> &str {
        match self {
            Translation::KJV => "King James Version",
            Translation::TOVBSI => "Tamil Old Version Bible Society of India",
        }
    }
    fn year(&self) -> u16 {
        match self {
            Translation::KJV => 1769,
            Translation::TOVBSI => 1957,
        }
    }
}

enum Testament {
    OldTestament,
    NewTestament,
}

struct Bible {
    language: Language,
    translation: Translation,
    description: Option<String>,
    books: Vec<Book>,
}

struct Book {
    bible: Bible,
    name: String,
    long_name: String,
    regional_name: String,
    regional_long_name: String,
    abbreviation: String,
    testament: Testament,
    description: Option<String>,
    chapters: Vec<Chapter>,
}

struct Chapter {
    book: Book,
    number: u8,
    description: Option<String>,
    verses: Vec<Verse>,
}

struct Verse {
    chapter: Chapter,
    number: u8,
    text: String,
    references: Option<Vec<Verse>>,
}

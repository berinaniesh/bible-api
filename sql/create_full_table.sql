create table fulltable as select vv.id as id, t.name as translation, b.name as book, b.abbreviation as abbreviation, tt.name as book_name, c.chapter_number as chapter, v.verse_number as verse_number, verse from "VerseText" vv join "Translation" t on vv.translation_id=t.id join "Verse" v on v.id=vv.verse_id join "Chapter" c on v.chapter_id=c.id join "Book" b on c.book_id=b.id join "TranslationBookName" tt on (t.id=tt.translation_id and b.id=tt.book_id) order by vv.id;
create index on fulltable(translation);
create index on fulltable(book);
create index on fulltable(book_name);
create index on fulltable(chapter);
create index on fulltable(verse_number);

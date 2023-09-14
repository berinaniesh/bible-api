CREATE TYPE "Testament" AS ENUM (
  'OldTestament',
  'NewTestament'
);

CREATE TYPE "Division" AS ENUM (
  'Pentateuch',
  'HistoricalBook',
  'WisdomBook',
  'MajorProphet',
  'MinorProphet',
  'Gospel',
  'History',
  'PaulineEpistle',
  'GeneralEpistle',
  'Prophecy'
);

CREATE TABLE "Language" (
  "id" SERIAL PRIMARY KEY,
  "name" varchar UNIQUE NOT NULL
);

CREATE TABLE "Translation" (
  "id" SERIAL PRIMARY KEY,
  "language_id" integer NOT NULL,
  "name" varchar UNIQUE NOT NULL,
  "full_name" varchar UNIQUE NOT NULL,
  "year" varchar,
  "license" varchar,
  "description" text
);

CREATE TABLE "Book" (
  "id" SERIAL PRIMARY KEY,
  "name" varchar NOT NULL,
  "long_name" varchar NOT NULL,
  "book_number" integer NOT NULL,
  "abbreviation" varchar,
  "testament" "Testament",
  "division" "Division",
  "description" text
);

CREATE TABLE "TranslationBookName" (
  "id" SERIAL PRIMARY KEY,
  "translation_id" integer NOT NULL,
  "book_id" integer NOT NULL,
  "name" varchar NOT NULL,
  "long_name" varchar
);

CREATE TABLE "Chapter" (
  "id" SERIAL PRIMARY KEY,
  "book_id" integer NOT NULL,
  "chapter_number" integer NOT NULL,
  "description" text
);

CREATE TABLE "Verse" (
  "id" SERIAL PRIMARY KEY,
  "chapter_id" integer NOT NULL,
  "verse_number" integer NOT NULL
);

CREATE TABLE "VerseText" (
  "id" SERIAL PRIMARY KEY,
  "translation_id" integer NOT NULL,
  "verse_id" integer NOT NULL,
  "verse" varchar NOT NULL
);

ALTER TABLE "Translation" ADD FOREIGN KEY ("language_id") REFERENCES "Language" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "TranslationBookName" ADD FOREIGN KEY ("translation_id") REFERENCES "Translation" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "TranslationBookName" ADD FOREIGN KEY ("book_id") REFERENCES "Book" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "Chapter" ADD FOREIGN KEY ("book_id") REFERENCES "Book" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "Verse" ADD FOREIGN KEY ("chapter_id") REFERENCES "Chapter" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "VerseText" ADD FOREIGN KEY ("translation_id") REFERENCES "Translation" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "VerseText" ADD FOREIGN KEY ("verse_id") REFERENCES "Verse" ("id") ON DELETE CASCADE ON UPDATE CASCADE;


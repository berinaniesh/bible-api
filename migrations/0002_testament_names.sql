CREATE TABLE "TestamentName" (
  "id" SERIAL PRIMARY KEY,
  "translation_id" integer NOT NULL,
  "testament" "Testament" NOT NULL,
  "name" varchar NOT NULL
);

ALTER TABLE "TestamentName" ADD FOREIGN KEY ("translation_id") REFERENCES "Translation" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

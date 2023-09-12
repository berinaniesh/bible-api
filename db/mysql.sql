CREATE TABLE `Language` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `name` varchar(255) UNIQUE NOT NULL
);

CREATE TABLE `Translation` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `language_id` integer UNIQUE NOT NULL,
  `name` varchar(255) UNIQUE NOT NULL,
  `full_name` varchar(255) UNIQUE,
  `year` varchar[4],
  `description` text
);

CREATE TABLE `Book` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `translation_id` integer UNIQUE NOT NULL,
  `name` varchar(255) NOT NULL,
  `long_name` varchar(255) NOT NULL,
  `regional_name` varchar(255),
  `regional_long_name` varchar(255),
  `book_number` integer NOT NULL,
  `abbreviation` varchar(255),
  `testament` ENUM ('OldTestament', 'NewTestament'),
  `division` ENUM ('Pentateuch', 'HistoricalBook', 'WisdomBook', 'MajorProphet', 'MinorProphet', 'Gospel', 'History', 'PaulineEpistle', 'GeneralEpistle', 'Prophecy'),
  `description` text
);

CREATE TABLE `Chapter` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `book_id` integer UNIQUE NOT NULL,
  `chapter_number` integer NOT NULL,
  `description` text
);

CREATE TABLE `Verse` (
  `id` integer PRIMARY KEY AUTO_INCREMENT,
  `chapter_id` integer UNIQUE NOT NULL,
  `verse_number` integer,
  `verse` varchar(255) NOT NULL
);

ALTER TABLE `Language` ADD FOREIGN KEY (`id`) REFERENCES `Translation` (`language_id`) ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE `Translation` ADD FOREIGN KEY (`id`) REFERENCES `Book` (`translation_id`) ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE `Book` ADD FOREIGN KEY (`id`) REFERENCES `Chapter` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE `Chapter` ADD FOREIGN KEY (`id`) REFERENCES `Verse` (`chapter_id`) ON DELETE CASCADE ON UPDATE CASCADE;

-- Create a database and tables
PRAGMA foreign_keys = ON;

-- Table: title_basics
CREATE TABLE title_basics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tconst TEXT UNIQUE NOT NULL,
    titleType TEXT,
    primaryTitle TEXT,
    originalTitle TEXT,
    isAdult BOOLEAN,
    startYear INTEGER,
    endYear INTEGER,
    runtimeMinutes INTEGER,
    genres TEXT
);

-- Table: title_crew
CREATE TABLE title_crew (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title_id INTEGER,
    directors TEXT,
    writers TEXT,
    FOREIGN KEY (title_id) REFERENCES title_basics (id)
);

-- Table: title_episode
CREATE TABLE title_episode (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    episode_id INTEGER,
    parent_id INTEGER,
    seasonNumber INTEGER,
    episodeNumber INTEGER,
    FOREIGN KEY (episode_id) REFERENCES title_basics (id),
    FOREIGN KEY (parent_id) REFERENCES title_basics (id)
);

-- Table: title_principals
CREATE TABLE title_principals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title_id INTEGER,
    ordering INTEGER,
    person_id INTEGER,
    category TEXT,
    job TEXT,
    characters TEXT,
    FOREIGN KEY (title_id) REFERENCES title_basics (id),
    FOREIGN KEY (person_id) REFERENCES name_basics (id)
);

-- Table: title_ratings
CREATE TABLE title_ratings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title_id INTEGER,
    averageRating REAL,
    numVotes INTEGER,
    FOREIGN KEY (title_id) REFERENCES title_basics (id)
);

-- Table: name_basics
CREATE TABLE name_basics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nconst TEXT UNIQUE NOT NULL,
    primaryName TEXT,
    birthYear INTEGER,
    deathYear INTEGER,
    primaryProfession TEXT,
    knownForTitles TEXT
);

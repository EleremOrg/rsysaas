CREATE TABLE IF NOT EXISTS web_sessions (
    session_id TEXT PRIMARY KEY NOT NULL,
    user_pk INTEGER,
    groups TEXT NOT NULL,
    last_accessed INTEGER NOT NULL,
    expiration INTEGER NOT NULL,
    csrf_token TEXT NOT NULL,
    data BLOB,
    country TEXT NOT NULL
);
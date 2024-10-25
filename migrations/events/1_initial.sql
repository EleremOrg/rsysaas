CREATE TABLE IF NOT EXISTS events (
    pk INTEGER PRIMARY KEY,
    source TEXT NOT NULL,
    command TEXT NOT NULL,
    version TEXT NOT NULL,
    priority INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    payload BLOB NOT NULL
);

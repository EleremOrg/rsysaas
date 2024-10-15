CREATE TABLE IF NOT EXISTS users (
    pk INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    activated_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS groups (
    pk INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users_groups_m2m (
    user_pk INTEGER NOT NULL,
    group_pk INTEGER NOT NULL,
    PRIMARY KEY (user_pk, group_pk),
    FOREIGN KEY (user_pk) REFERENCES users (pk),
    FOREIGN KEY (group_pk) REFERENCES groups (pk)
);

CREATE TABLE IF NOT EXISTS tokens (
    pk INTEGER PRIMARY KEY,
    token TEXT NOT NULL UNIQUE,
    user_pk INTEGER NOT NULL,
    FOREIGN KEY (user_pk) REFERENCES users (pk)
);

CREATE TABLE IF NOT EXISTS users (
    pk INTEGER PRIMARY KEY,
    password TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS emails (
    pk INTEGER PRIMARY KEY,
    is_primary INTEGER NOT NULL DEFAULT 0,
    user_pk INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    activated_at INTEGER,
    email TEXT NOT NULL UNIQUE,
    FOREIGN KEY (user_pk) REFERENCES users (pk)
);

CREATE TABLE IF NOT EXISTS groups (
    pk INTEGER PRIMARY KEY,
    created_at INTEGER NOT NULL,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS users_groups_m2m (
    user_pk INTEGER NOT NULL,
    group_pk INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (user_pk, group_pk),
    FOREIGN KEY (user_pk) REFERENCES users (pk),
    FOREIGN KEY (group_pk) REFERENCES groups (pk)
);

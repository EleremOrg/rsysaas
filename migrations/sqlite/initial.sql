CREATE TABLE customers (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT NOT NULL,
    api_key TEXT NOT NULL,
    UNIQUE(api_key),
    UNIQUE(domain)
);
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    customer_id INTEGER,
    FOREIGN KEY (customer_id) REFERENCES customers (id)
);
CREATE TABLE associations (
    id INTEGER PRIMARY KEY,
    table_related TEXT NOT NULL,
    row_id INTEGER
);

CREATE TABLE companies (
    id INTEGER PRIMARY KEY,
    ticker TEXT NOT NULL,
    sector TEXT NOT NULL,
    industry TEXT NOT NULL,
    exchange TEXT NOT NULL,
    country TEXT NOT NULL,
    adj TEXT,
    growth REAL
);

CREATE TABLE terms (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    category TEXT NOT NULL,
    tags TEXT NOT NULL
);

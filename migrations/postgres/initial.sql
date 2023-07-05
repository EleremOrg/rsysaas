CREATE TABLE customers (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT NOT NULL,
    api_key TEXT NOT NULL,
    CONSTRAINT api_key_unique UNIQUE(api_key),
    CONSTRAINT domain_unique UNIQUE(domain)
);
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    customer_id INTEGER,
    CONSTRAINT fk_customer_id FOREIGN KEY (customer_id) REFERENCES customers (id)
);
CREATE TABLE associations (
    id SERIAL PRIMARY KEY,
    table_related TEXT NOT NULL,
    row_id INTEGER
);

CREATE TABLE companies (
    id SERIAL PRIMARY KEY,
    ticker TEXT NOT NULL,
    sector TEXT NOT NULL,
    industry TEXT NOT NULL,
    exchange TEXT NOT NULL,
    country TEXT NOT NULL,
    adj TEXT[],
    growth REAL
);

CREATE TABLE terms (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    category TEXT NOT NULL,
    tags TEXT NOT NULL
);

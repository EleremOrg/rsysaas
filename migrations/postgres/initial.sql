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

INSERT INTO company (id, ticker, sector, industry, exchange, country, adj, growth)
VALUES
    (1, 'AAPL', 'Technology', 'Healthcare', 'NASDAQ', 'USA', ARRAY['growth', 'divs']::VARCHAR(255)[], 0.1),
    (2, 'GOOGL', 'Technology', 'Telecommunications', 'NASDAQ', 'USA', ARRAY['value', 'growth', 'divs']::VARCHAR(255)[], 0.5),
    (3, 'MSFT', 'Technology', 'Finance', 'NASDAQ', 'USA', ARRAY['growth', 'zombie']::VARCHAR(255)[], 0.3),
    (4, 'AMZN', 'Technology', 'Retail', 'NASDAQ', 'USA', ARRAY['value', 'growth', 'divs', 'zombie']::VARCHAR(255)[], 0.25),
    (5, 'FB', 'Technology', 'Media', 'NASDAQ', 'USA', ARRAY['growth', 'divs', 'zombie']::VARCHAR(255)[], 0.12),
    (6, 'TSLA', 'Automotive', 'Automotive', 'NASDAQ', 'USA', ARRAY['growth']::VARCHAR(255)[], 0.33),
    (7, 'JPM', 'Financial Services', 'Unknown', 'NYSE', 'USA', ARRAY['value', 'zombie']::VARCHAR(255)[], 0.05),
    (8, 'BAC', 'Financial Services', 'Unknown', 'NYSE', 'USA', ARRAY['growth', 'zombie']::VARCHAR(255)[], 0.09),
    (9, 'WMT', 'Consumer Cyclical', 'Unknown', 'NYSE', 'USA', ARRAY['value', 'divs']::VARCHAR(255)[], 0.012),
    (10, 'GE', 'Automotive', 'Unknown', 'NYSE', 'USA', ARRAY['growth', 'zombie']::VARCHAR(255)[], -0.1),
    (12, 'FIRST', 'Technology', 'Technology', 'NASDAQ', 'USA', ARRAY['growth', 'divs']::VARCHAR(255)[], 0.3);
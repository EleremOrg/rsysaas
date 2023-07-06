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

INSERT INTO companies (id, ticker, sector, industry, exchange, country, adj, growth)
VALUES
    (1, 'AAPL', 'Technology', 'Healthcare', 'NASDAQ', 'USA', 'growth,divs', 0.1),
    (2, 'GOOGL', 'Technology', 'Telecommunications', 'NASDAQ', 'USA', 'value,growth,divs', 0.5),
    (3, 'MSFT', 'Technology', 'Finance', 'NASDAQ', 'USA', 'growth,zombie', 0.3),
    (4, 'AMZN', 'Technology', 'Retail', 'NASDAQ', 'USA', 'value,growth,divs,zombie', 0.25),
    (5, 'FB', 'Technology', 'Media', 'NASDAQ', 'USA', 'growth,divs,zombie', 0.12),
    (6, 'TSLA', 'Automotive', 'Automotive', 'NASDAQ', 'USA', 'growth', 0.33),
    (7, 'JPM', 'Financial Services', 'Unknown', 'NYSE', 'USA', 'value,zombie', 0.05),
    (8, 'BAC', 'Financial Services', 'Unknown', 'NYSE', 'USA', 'growth,zombie', 0.09),
    (9, 'WMT', 'Consumer Cyclical', 'Unknown', 'NYSE', 'USA', 'value,divs', 0.012),
    (10, 'GE', 'Automotive', 'Unknown', 'NYSE', 'USA', 'growth,zombie', -0.1),
    (12, 'FIRST', 'Technology', 'Technology', 'NASDAQ', 'USA', 'growth,divs', 0.3);

INSERT INTO customers (id, name, domain, api_key)
VALUES
    (1, 'Acme Corporation', 'acme.com', 'acme_api_key'),
    (2, 'Widget Inc.', 'widget.com', 'widget_api_key');

INSERT INTO users (id, name, customer_id)
VALUES
    (1, 'John Doe', 1),
    (2, 'Jane Smith', 2),
    (3, 'Robert Johnson', 1);

INSERT INTO terms (id, title, slug, category, tags) VALUES
(1, 'Term 1', 'term-1', 'Category 1', 'Tag1, Tag2, Tag3'),
(2, 'Term 2', 'term-2', 'Category 2', 'Tag4, Tag5'),
(3, 'Term 3', 'term-3', 'Category 1', 'Tag2, Tag6, Tag7'),
(4, 'Term 4', 'term-4', 'Category 3', 'Tag8'),
(5, 'Term 5', 'term-5', 'Category 2', 'Tag9, Tag10'),
(6, 'Term 6', 'term-6', 'Category 3', 'Tag11, Tag12'),
(7, 'Term 7', 'term-7', 'Category 1', 'Tag13, Tag14, Tag15'),
(8, 'Term 8', 'term-8', 'Category 2', 'Tag5, Tag16'),
(9, 'Term 9', 'term-9', 'Category 3', 'Tag17'),
(10, 'Term 10', 'term-10', 'Category 1', 'Tag18, Tag19, Tag20');
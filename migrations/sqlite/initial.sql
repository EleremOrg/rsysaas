CREATE TABLE migrations (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    ran BOOLEAN DEFAULT false,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    customer_id INTEGER,
    FOREIGN KEY (customer_id) REFERENCES customers (id)
);

CREATE TABLE potential_customers (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    message TEXT,
    agent TEXT,
    language TEXT,
    url TEXT
);

CREATE TABLE customers (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT NOT NULL,
    token TEXT NOT NULL,
    public_token TEXT NOT NULL,
    models_related TEXT NOT NULL,
    UNIQUE(token),
    UNIQUE(public_token),
    UNIQUE(domain)
);

CREATE TABLE recommendations_responses (
    id INTEGER PRIMARY KEY,
    request_id INTEGER,
    request_type TEXT NOT NULL,
    main_item_id INTEGER,
    main_item_entity TEXT NOT NULL,
    entity_id INTEGER,
    entity TEXT NOT NULL,
    image TEXT NOT NULL,
    title TEXT NOT NULL,
    resume TEXT NOT NULL,
    score REAL,
    algorithm TEXT NOT NULL,
    url TEXT NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    customer_id INTEGER,
    FOREIGN KEY (customer_id) REFERENCES customers (id)
);

CREATE TABLE recommendations_used (
    id INTEGER PRIMARY KEY,
    created_at TIMESTAMP,
    recommendation_response_id INTEGER,
    FOREIGN KEY (recommendation_response_id) REFERENCES recommendations_responses (id)
);

CREATE TABLE embed_recommendation_requests (
    id INTEGER PRIMARY KEY,
    orientation TEXT,
    entity TEXT,
    title TEXT,
    show_image BOOLEAN,
    show_resume BOOLEAN,
    user_id INTEGER,
    prod_id INTEGER,
    number_recommendations INTEGER,
    is_transparent BOOLEAN,
    height INTEGER,
    width INTEGER,
    locale TEXT,
    color_theme TEXT,
    public_key TEXT,
    location_href TEXT,
    base_uri TEXT,
    doc_url TEXT,
    user_agent TEXT,
    language TEXT,
    languages TEXT,
    screen_width INTEGER,
    screen_height INTEGER,
    referrer TEXT,
    document_title TEXT,
    host TEXT,
    location TEXT,
    customer_id INTEGER,
    FOREIGN KEY (customer_id) REFERENCES customers (id)
);

CREATE TABLE api_recommendation_request (
    id INTEGER PRIMARY KEY,
    entity TEXT,
    user_id INTEGER,
    prod_id INTEGER,
    number_recommendations INTEGER,
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
    resume TEXT NOT NULL,
    image TEXT NOT NULL,
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
    resume TEXT NOT NULL,
    image TEXT NOT NULL,
    slug TEXT NOT NULL,
    category TEXT NOT NULL,
    tags TEXT NOT NULL
);

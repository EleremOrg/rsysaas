CREATE TABLE IF NOT EXISTS countries (
    pk INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_countries_code ON countries(code);

CREATE TABLE IF NOT EXISTS currencies (
    pk INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_currencies_code ON currencies(code);

CREATE TABLE IF NOT EXISTS customers_companies (
    pk INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT NOT NULL,
    description TEXT NOT NULL,
    country_pk INTEGER NOT NULL,
    currency_pk INTEGER,
    created_at INTEGER NOT NULL,
    parent_company_pk INTEGER,
    FOREIGN KEY (parent_company_pk) REFERENCES customers_companies (pk),
    FOREIGN KEY (currency_pk) REFERENCES currencies (pk),
    FOREIGN KEY (country_pk) REFERENCES countries (pk)
);

CREATE TABLE IF NOT EXISTS customers (
    pk INTEGER PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    user_pk INTEGER NOT NULL,
    FOREIGN KEY (user_pk) REFERENCES users (pk)
);

CREATE TABLE IF NOT EXISTS customers_companies_m2m (
    customer_pk INTEGER NOT NULL,
    company_pk INTEGER NOT NULL,
    PRIMARY KEY (customer_pk, company_pk),
    FOREIGN KEY (customer_pk) REFERENCES customers (pk),
    FOREIGN KEY (company_pk) REFERENCES customers_companies (pk)
);


CREATE TABLE IF NOT EXISTS shopify_shops (
    pk INTEGER PRIMARY KEY,
    id TEXT NOT NULL UNIQUE,
    shop TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    activated_at INTEGER,
    unit_system TEXT NOT NULL,
    weight_unit TEXT NOT NULL,
    company_pk INTEGER NOT NULL,
    FOREIGN KEY (company_pk) REFERENCES customers_companies (pk)
);

CREATE UNIQUE INDEX idx_shopify_shops_shop ON shopify_shops(shop);

CREATE TABLE IF NOT EXISTS shopify_shop_tokens (
    pk INTEGER PRIMARY KEY,
    shopify_shop_pk INTEGER NOT NULL,
    token TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    scope TEXT NOT NULL,
    FOREIGN KEY (shopify_shop_pk) REFERENCES shopify_shops (pk)
);


CREATE TABLE IF NOT EXISTS products (
    pk INTEGER PRIMARY KEY,
    id TEXT NOT NULL,
    company_pk INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    meta BLOB NOT NULL,
    FOREIGN KEY (company_pk) REFERENCES customers_companies (pk),
    UNIQUE(id, company_pk)
);

CREATE TABLE IF NOT EXISTS orders (
    pk INTEGER PRIMARY KEY,
    id TEXT NOT NULL,
    company_pk INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    product_pk INTEGER,
    FOREIGN KEY (company_pk) REFERENCES customers_companies (pk),
    FOREIGN KEY (product_pk) REFERENCES products (pk),
    UNIQUE(id, company_pk)
);

CREATE TABLE IF NOT EXISTS refunds (
    pk INTEGER PRIMARY KEY,
    reason TEXT NOT NULL DEFAULT "",
    id TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    company_pk INTEGER NOT NULL,
    order_pk INTEGER,
    FOREIGN KEY (company_pk) REFERENCES customers_companies (pk),
    FOREIGN KEY (order_pk) REFERENCES orders (pk),
    UNIQUE(id, company_pk)
);

CREATE UNIQUE INDEX idx_products_id ON products(id, company_pk);
CREATE UNIQUE INDEX idx_orders_id ON orders(id, company_pk);
CREATE UNIQUE INDEX idx_refunds_id ON refunds(id, company_pk);
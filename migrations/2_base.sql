CREATE TABLE IF NOT EXISTS customers_companies (
    pk INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT NOT NULL,
    parent_company_pk INTEGER,
    FOREIGN KEY (parent_company_pk) REFERENCES customers_companies (pk)
);

CREATE TABLE IF NOT EXISTS shopify_profiles (
    pk INTEGER PRIMARY KEY,
    shop TEXT NOT NULL UNIQUE,
    token TEXT NOT NULL,
    created_at TEXT NOT NULL,
    scope TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS shopify_profiles_customers_companies_m2m (
    shopify_profile_pk INTEGER NOT NULL,
    company_pk INTEGER NOT NULL,
    PRIMARY KEY (shopify_profile_pk, company_pk),
    FOREIGN KEY (shopify_profile_pk) REFERENCES shopify_profiles (pk),
    FOREIGN KEY (company_pk) REFERENCES customers_companies (pk)
);

CREATE TABLE IF NOT EXISTS customers (
    pk INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
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


CREATE TABLE IF NOT EXISTS products (
    pk INTEGER PRIMARY KEY,
    id TEXT NOT NULL,
    company_pk INTEGER NOT NULL,
    meta BLOB NOT NULL,
    FOREIGN KEY (company_pk) REFERENCES customers_companies (pk),
    UNIQUE(id, company_pk)
);

CREATE TABLE IF NOT EXISTS orders (
    pk INTEGER PRIMARY KEY,
    id TEXT NOT NULL,
    company_pk INTEGER NOT NULL,
    product_pk INTEGER,
    FOREIGN KEY (company_pk) REFERENCES customers_companies (pk),
    FOREIGN KEY (product_pk) REFERENCES products (pk),
    UNIQUE(id, company_pk)
);

CREATE TABLE IF NOT EXISTS refunds (
    pk INTEGER PRIMARY KEY,
    reason TEXT NOT NULL DEFAULT "",
    id TEXT NOT NULL,
    company_pk INTEGER NOT NULL,
    order_pk INTEGER,
    FOREIGN KEY (company_pk) REFERENCES customers_companies (pk),
    FOREIGN KEY (order_pk) REFERENCES orders (pk),
    UNIQUE(id, company_pk)
);

CREATE UNIQUE INDEX idx_products_id ON products(id, company_pk);
CREATE UNIQUE INDEX idx_orders_id ON orders(id, company_pk);
CREATE UNIQUE INDEX idx_refunds_id ON refunds(id, company_pk);
CREATE UNIQUE INDEX idx_shopify_profiles_shop ON shopify_profiles(shop);
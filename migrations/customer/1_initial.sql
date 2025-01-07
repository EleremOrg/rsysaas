CREATE TABLE IF NOT EXISTS products (
    pk INTEGER PRIMARY KEY,
    id UNIQUE TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS products_variants (
    pk INTEGER PRIMARY KEY,
    id TEXT NOT NULL,
    product_pk INTEGER,
    FOREIGN KEY (product_pk) REFERENCES products (pk),
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS products_informations (
    pk INTEGER PRIMARY KEY,
    price: TEXT NOT NULL,
    currency: INTEGER NOT NULL,
    image: TEXT,
    url: TEXT NOT NULL,
    specifications_table: TEXT NOT NULL,
    specifications_pk: INTEGER NOT NULL,
    variant_pk INTEGER,
    FOREIGN KEY (variant_pk) REFERENCES products_variants (pk),
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS customers (
    pk INTEGER PRIMARY KEY,
    id UNIQUE TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS orders (
    pk INTEGER PRIMARY KEY,
    id UNIQUE TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    variant_pk INTEGER,
    FOREIGN KEY (variant_pk) REFERENCES products_variants (pk)
);

CREATE TABLE IF NOT EXISTS refunds (
    pk INTEGER PRIMARY KEY,
    reason TEXT NOT NULL DEFAULT "",
    id UNIQUE TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    order_pk INTEGER,
    variant_pk INTEGER,
    FOREIGN KEY (variant_pk) REFERENCES products_variants (pk),
    FOREIGN KEY (order_pk) REFERENCES orders (pk)
);

CREATE UNIQUE INDEX idx_customers_id ON customers(id);
CREATE UNIQUE INDEX idx_products_id ON products(id);
CREATE UNIQUE INDEX idx_orders_id ON orders(id);
CREATE UNIQUE INDEX idx_refunds_id ON refunds(id);
CREATE TABLE IF NOT EXISTS products (
    pk INTEGER PRIMARY KEY,
    id UNIQUE TEXT NOT NULL,
    price: TEXT NOT NULL,
    currency: INTEGER NOT NULL,
    image: TEXT,
    url: TEXT NOT NULL,
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
    company_pk INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    product_pk INTEGER,
    customer_pk INTEGER,
    FOREIGN KEY (customer_pk) REFERENCES customers (pk),
    FOREIGN KEY (product_pk) REFERENCES products (pk)
);

CREATE TABLE IF NOT EXISTS refunds (
    pk INTEGER PRIMARY KEY,
    reason TEXT NOT NULL DEFAULT "",
    id UNIQUE TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    company_pk INTEGER NOT NULL,
    order_pk INTEGER,
    FOREIGN KEY (order_pk) REFERENCES orders (pk)
);

CREATE UNIQUE INDEX idx_customers_id ON customers(id);
CREATE UNIQUE INDEX idx_products_id ON products(id);
CREATE UNIQUE INDEX idx_orders_id ON orders(id);
CREATE UNIQUE INDEX idx_refunds_id ON refunds(id);
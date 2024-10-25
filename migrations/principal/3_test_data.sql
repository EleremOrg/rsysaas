-- Insert mock data for users
INSERT INTO users (pk, username, password, activated_at, created_at) VALUES
(1, 'john_doe', 'password123', strftime('%s','now') - 100000, strftime('%s','now') - 200000),
(2, 'jane_smith', 'password456', strftime('%s','now') - 50000, strftime('%s','now') - 150000),
(3, 'alice_jones', 'password789', NULL, strftime('%s','now') - 100000);

-- Insert mock data for emails
INSERT INTO emails (pk, is_primary, user_pk, created_at, activated_at, email) VALUES
(1, 1, 1, strftime('%s','now') - 190000, strftime('%s','now') - 180000, 'john_doe@example.com'),
(2, 1, 2, strftime('%s','now') - 140000, strftime('%s','now') - 130000, 'jane_smith@example.com'),
(3, 0, 1, strftime('%s','now') - 170000, NULL, 'john_doe_alt@example.com'),
(4, 1, 3, strftime('%s','now') - 90000, NULL, 'alice_jones@example.com');

-- Insert mock data for groups
INSERT INTO groups (pk, created_at, name) VALUES
(1, strftime('%s','now') - 300000, 'Admins'),
(2, strftime('%s','now') - 250000, 'Editors'),
(3, strftime('%s','now') - 200000, 'Viewers');

-- Insert mock data for users_groups_m2m
INSERT INTO users_groups_m2m (user_pk, group_pk, created_at) VALUES
(1, 1, strftime('%s','now') - 190000),
(1, 2, strftime('%s','now') - 190000),
(2, 3, strftime('%s','now') - 140000),
(3, 2, strftime('%s','now') - 90000);

-- Insert mock data for customers_companies
INSERT INTO customers_companies (pk, name, domain, created_at, parent_company_pk) VALUES
(1, 'TechCorp', 'techcorp.com', strftime('%Y-%m-%d %H:%M:%S', 'now') - 300000, NULL),
(2, 'DesignStudio', 'designstudio.com', strftime('%Y-%m-%d %H:%M:%S', 'now') - 250000, NULL),
(3, 'SoftSolutions', 'softsolutions.com', strftime('%Y-%m-%d %H:%M:%S', 'now') - 200000, 1);

-- Insert mock data for customers
INSERT INTO customers (pk, name, email, created_at, user_pk) VALUES
(1, 'Michael Brown', 'michael.brown@example.com', strftime('%s','now') - 150000, 1),
(2, 'Sarah White', 'sarah.white@example.com', strftime('%s','now') - 120000, 2),
(3, 'David Green', 'david.green@example.com', strftime('%s','now') - 110000, 3);

-- Insert mock data for customers_companies_m2m
INSERT INTO customers_companies_m2m (customer_pk, company_pk) VALUES
(1, 1),
(2, 2),
(3, 3);


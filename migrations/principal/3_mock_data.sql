INSERT INTO users (pk, password, is_active, created_at) VALUES 
(1, '$argon2id$v=19$m=19456,t=2,p=1$cI90PT+bCwS9/f9KMbWvzw$rYkFATX+P/ZNbVCvEMgQX3C1mmrAcU5vGmv6ohctk1g', 1, 1693440000),
(2, '$argon2id$v=19$m=19456,t=2,p=1$RvI4zhbP4z3JjpT2TndmZQ$afjYtgjzIou3vaGpyOfcKzHPHBYJ2RHFUDaDyu9Cu0o', 0, 1693526400),
(3, '$argon2id$v=19$m=19456,t=2,p=1$J0+xF62+j2XJB9Jx2FkClw$v3ChlsN7fUpniOXeR2Lmd0rKBnuM4xOHkSsnM0zZ9YA', 1, 1693612800);

-- Emails
INSERT INTO emails (pk, is_primary, user_pk, created_at, activated_at, email) VALUES 
(1, 1, 1, 1693440000, 1693526400, 'user1@example.com'),
(2, 0, 1, 1693440000, NULL, 'user1_secondary@example.com'),
(3, 1, 2, 1693526400, 1693612800, 'user2@example.com'),
(4, 1, 3, 1693612800, NULL, 'user3@example.com');

-- Groups
INSERT INTO groups (pk, created_at, name) VALUES 
(1, 1693440000, 'Admin'),
(2, 1693526400, 'Editor'),
(3, 1693612800, 'Viewer');

-- Users-Groups Many-to-Many
INSERT INTO users_groups_m2m (user_pk, group_pk, created_at) VALUES 
(1, 1, 1693440000),
(1, 2, 1693526400),
(2, 3, 1693612800),
(3, 1, 1693612800);

-- Countries
INSERT INTO countries (pk, name, code, created_at) VALUES 
(1, 'United States', 'US', 1693440000),
(2, 'Canada', 'CA', 1693526400),
(3, 'Mexico', 'MX', 1693612800);

-- Currencies
INSERT INTO currencies (pk, name, code, created_at) VALUES 
(1, 'US Dollar', 'USD', 1693440000),
(2, 'Canadian Dollar', 'CAD', 1693526400),
(3, 'Mexican Peso', 'MXN', 1693612800);

-- Customers Companies
INSERT INTO customers_companies (pk, name, domain, description, country_pk, currency_pk, created_at, parent_company_pk) VALUES 
(1, 'Tech Solutions', 'techsolutions.com', 'Tech consulting firm', 1, 1, 1693440000, NULL),
(2, 'North Services', 'northservices.ca', 'Canadian IT services', 2, 2, 1693526400, NULL),
(3, 'South Imports', 'southimports.mx', 'Importer of goods', 3, 3, 1693612800, NULL),
(4, 'Global Corp', 'globalcorp.com', 'International conglomerate', 1, 1, 1693612800, 1);

-- Customers
INSERT INTO customers (pk, first_name, last_name, created_at, user_pk) VALUES 
(1, 'John', 'Doe', 1693440000, 1),
(2, 'Jane', 'Smith', 1693526400, 2),
(3, 'Carlos', 'Hernandez', 1693612800, 3);

-- Customers-Companies Many-to-Many
INSERT INTO customers_companies_m2m (customer_pk, company_pk) VALUES 
(1, 1),
(1, 4),
(2, 2),
(3, 3),
(3, 4);
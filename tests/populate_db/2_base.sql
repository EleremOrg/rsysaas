-- Inserting mock data into the 'customers_companies' table
INSERT INTO customers_companies (name, domain, parent_company_pk) VALUES 
('Tech Corp', 'techcorp.com', NULL),  -- A standalone company
('Soft Solutions', 'softsolutions.com', NULL),  -- Another standalone company
('Tech Corp Subsidiary', 'subsidiary.techcorp.com', 1);  -- A subsidiary of Tech Corp

-- Inserting mock data into the 'customers' table
INSERT INTO customers (name, email, user_pk) VALUES 
('John Doe', 'john.doe@techcorp.com', 1),  -- John Doe works for 'Tech Corp'
('Jane Smith', 'jane.smith@softsolutions.com', 2),  -- Jane Smith works for 'Soft Solutions'
('Alice Wonderland', 'alice@subsidiary.techcorp.com', 3);  -- Alice works for 'Tech Corp Subsidiary'

-- Inserting mock data into the 'customers_companies_m2m' table (many-to-many relationship between customers and companies)
INSERT INTO customers_companies_m2m (customer_pk, company_pk) VALUES 
(1, 1),  -- John Doe is associated with 'Tech Corp'
(2, 2),  -- Jane Smith is associated with 'Soft Solutions'
(3, 3);  -- Alice Wonderland is associated with 'Tech Corp Subsidiary'

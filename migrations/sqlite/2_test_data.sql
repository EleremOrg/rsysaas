-- Insert data into customers table
INSERT INTO customers (name, domain, email, token, public_token, models_related)
VALUES
    ('InvFin', 'example.com:8000', 'master@InvFin.com', 'invfin_token_123', 'public_invfin_token_123', 'users,terms,companies'),
    ('XYZ Inc.', 'xyzinc.com', 'master@xyzinc.com', 'xyz_token_456', 'public_xyz_token_456', 'users'),
    ('Tech Solutions Ltd.', 'techsolutions.com', 'master@techsolutions.com', 'tech_token_789', 'public_tech_token_789', 'users');

-- Insert data into users table, associating them with their respective customers using customer_id
INSERT INTO users (name, customer_id)
VALUES
    ('John Doe', 1), -- John Doe belongs to Acme Corporation (customer_id 1)
    ('Jane Smith', 1), -- Jane Smith also belongs to Acme Corporation (customer_id 1)
    ('Mike Johnson', 2), -- Mike Johnson belongs to XYZ Inc. (customer_id 2)
    ('Sarah Lee', 3), -- Sarah Lee belongs to Tech Solutions Ltd. (customer_id 3)
    ('Mark Davis', 2), -- Mark Davis belongs to XYZ Inc. (customer_id 2)
    ('Emily Adams', 1), -- Emily Adams belongs to Acme Corporation (customer_id 1)
    ('David Chen', 3), -- David Chen belongs to Tech Solutions Ltd. (customer_id 3)
    ('Lisa Wang', 1), -- Lisa Wang belongs to Acme Corporation (customer_id 1)
    ('Andrew Brown', 2), -- Andrew Brown belongs to XYZ Inc. (customer_id 2)
    ('Jessica Lee', 3), -- Jessica Lee belongs to Tech Solutions Ltd. (customer_id 3)
    ('Michael Taylor', 1), -- Michael Taylor belongs to Acme Corporation (customer_id 1)
    ('Olivia Wilson', 2), -- Olivia Wilson belongs to XYZ Inc. (customer_id 2)
    ('William Hall', 3); -- William Hall belongs to Tech Solutions Ltd. (customer_id 3)


-- Sectors
INSERT INTO companies_sectors (name) VALUES
    ('Technology'),
    ('Finance'),
    ('Retail'),
    ('Consumer Goods'),
    ('Environment'),
    ('Business'),
    ('Science'),
    ('Medical');

-- Industries
INSERT INTO companies_industries (name) VALUES
    ('Consumer Electronics'),
    ('Internet Services'),
    ('Software'),
    ('E-Commerce'),
    ('Banking'),
    ('Supermarkets'),
    ('Conglomerate'),
    ('Financial Services'),
    ('FMCG'),
    ('Social Media'),
    ('Cryptocurrency'),
    ('Distributed Ledger'),
    ('Global Warming'),
    ('Greenhouse Gases'),
    ('Digital Payments'),
    ('Financial Innovation'),
    ('Renewable Energy'),
    ('Big Data'),
    ('Data Analysis'),
    ('Online Shopping'),
    ('Internet Retail'),
    ('Biotechnology'),
    ('Gene Editing'),
    ('Medical Services');

INSERT INTO terms (title, resume, image, path, category, tags)
VALUES
    ('Artificial Intelligence', 'Artificial intelligence (AI) is...', 'https://logo.clearbit.com/ai.com', 'definicion/artificial-intelligence', 'Technology', 'AI, Machine Learning'),
    ('Blockchain', 'Blockchain is a decentralized...', 'https://logo.clearbit.com/blockchain.com', 'definicion/blockchain', 'Technology', 'Cryptocurrency, Distributed Ledger'),
    ('Climate Change', 'Climate change refers to...', 'https://logo.clearbit.com/climatechange.com', 'definicion/climate-change', 'Environment', 'Global Warming, Greenhouse Gases'),
    ('P/B', 'price to book is really cool ...', 'https://logo.clearbit.com/fintech.com', 'definicion/precio-valor-en-libros', 'Finance', 'Digital Payments, Financial Innovation'),
    ('Sustainability', 'Sustainability is the practice...', 'https://logo.clearbit.com/sustainability.com', 'definicion/sustainability', 'Environment', 'Renewable Energy, Eco-friendly'),
    ('Data Science', 'Data science is an interdisciplinary...', 'https://logo.clearbit.com/datascience.com', 'definicion/data-science', 'Technology', 'Big Data, Data Analysis'),
    ('E-commerce', 'E-commerce (Electronic Commerce)...', 'https://logo.clearbit.com/ecommerce.com', 'definicion/e-commerce', 'Business', 'Online Shopping, Internet Retail'),
    ('Genetic Engineering', 'Genetic engineering is a...', 'https://logo.clearbit.com/geneticengineering.com', 'definicion/genetic-engineering', 'Science', 'Biotechnology, Gene Editing'),
    ('Healthcare', 'Healthcare is the maintenance...', 'https://logo.clearbit.com/healthcare.com', 'definicion/healthcare', 'Medical', 'Medical Services, Health Industry'),
    ('Robotics', 'Robotics is the branch of...', 'https://logo.clearbit.com/robotics.com', 'definicion/robotics', 'Technology', 'Automation, Robots');

-- Companies data with sector_id and industry_id
INSERT INTO companies (ticker, path, resume, image, sector_id, industry_id, exchange, country, adj, growth)
VALUES
    ('MSFT', 'screener/analisis-de/MSFT/' ,'Microsoft Corporation is a leading...', 'https://logo.clearbit.com/microsoft.com', 1, 3, 'NASDAQ', 'United States', 'Yes', 0.10),
    ('AMZN', 'screener/analisis-de/AMZN/' ,'Amazon.com, Inc. is a multinational...', 'https://logo.clearbit.com/amazon.com', 1, 4, 'NASDAQ', 'United States', 'Yes', 0.18),
    ('JPM', 'screener/analisis-de/JPM/' ,'JPMorgan Chase & Co. is a global...', 'https://logo.clearbit.com/jpmorganchase.com', 2, 5, 'NYSE', 'United States', 'Yes', 0.05),
    ('WMT', 'screener/analisis-de/WMT/' ,'Walmart Inc. is a multinational...', 'https://logo.clearbit.com/walmart.com', 3, 6, 'NYSE', 'United States', 'No', 0.08),
    ('BRK', 'screener/analisis-de/BRK/' ,'Berkshire Hathaway Inc. is a multinational...', 'https://logo.clearbit.com/berkshirehathaway.com', 2, 7, 'NYSE', 'United States', 'No', 0.04),
    ('V', 'screener/analisis-de/V/' ,'Visa Inc. is a global payments...', 'https://logo.clearbit.com/visa.com', 2, 8, 'NYSE', 'United States', 'Yes', 0.14),
    ('PG', 'screener/analisis-de/PG/' ,'Procter & Gamble Co. is a multinational...', 'https://logo.clearbit.com/pg.com', 4, 9, 'NYSE', 'United States', 'Yes', 0.06),
    ('FB', 'screener/analisis-de/FB/' ,'Meta Platforms, Inc. (formerly Facebook, Inc.) is a...', 'https://logo.clearbit.com/meta.com', 1, 10, 'NASDAQ', 'United States', 'Yes', 0.20),
    ('TSLA', 'screener/analisis-de/TSLA/' ,'Tesla, Inc. is an electric vehicle and clean energy...', 'https://logo.clearbit.com/tesla.com', 1, 4, 'NASDAQ', 'United States', 'Yes', 0.25),
    ('AAP', 'screener/analisis-de/AAP/' ,'Advance Auto Parts, Inc. is a retailer...', 'https://logo.clearbit.com/advanceautoparts.com', 3, 6, 'NYSE', 'United States', 'No', 0.05);

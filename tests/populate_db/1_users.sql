-- Inserting mock data into the 'users' table
INSERT INTO users (username, email, password, activated_at, created_at) VALUES 
('john_doe', 'john@example.com', 'password123', '2024-10-01 12:30:00', '2024-09-25 14:00:00'),
('jane_smith', 'jane@example.com', 'password456', '2024-10-02 09:15:00', '2024-09-20 11:30:00'),
('alice_wonder', 'alice@example.com', 'password789', NULL, '2024-09-27 15:45:00');

-- Inserting mock data into the 'groups' table
INSERT INTO groups (name) VALUES 
('admin'),
('editor'),
('viewer');

-- Inserting mock data into the 'users_groups_m2m' table (many-to-many relationship between users and groups)
INSERT INTO users_groups_m2m (user_pk, group_pk) VALUES 
(1, 1),  -- John is in 'admin' group
(2, 2),  -- Jane is in 'editor' group
(2, 3),  -- Jane is also in 'viewer' group
(3, 3);  -- Alice is in 'viewer' group

-- Inserting mock data into the 'tokens' table
INSERT INTO tokens (token, user_pk) VALUES 
('abc123token', 1),  -- Token for John
('def456token', 2),  -- Token for Jane
('ghi789token', 3);  -- Token for Alice

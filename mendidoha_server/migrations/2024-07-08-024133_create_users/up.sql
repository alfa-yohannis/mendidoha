-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(10) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(100) NOT NULL
);
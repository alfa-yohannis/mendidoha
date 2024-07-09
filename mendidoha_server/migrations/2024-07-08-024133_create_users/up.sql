CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(10) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL
);

INSERT INTO users (user_id, username, password) 
    VALUES ('admin001', 'admin', '1234');
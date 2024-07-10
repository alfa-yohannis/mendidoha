CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(10) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(32) NOT NULL, -- Length of MD5 hash is 32 characters
    first_name VARCHAR(50) NOT NULL,
    middle_name VARCHAR(50),
    last_name VARCHAR(50) NOT NULL,
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Example using MD5 for password hashing
INSERT INTO users (user_id, username, password, first_name, middle_name, last_name) 
    VALUES ('admin001', 'admin', md5('1234'), 'Admin', 'Middle', 'User');

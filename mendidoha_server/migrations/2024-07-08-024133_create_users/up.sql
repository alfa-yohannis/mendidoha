CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(32) NOT NULL, -- Length of MD5 hash is 32 characters
    first_name VARCHAR(50) NOT NULL,
    middle_name VARCHAR(50),
    last_name VARCHAR(50) NOT NULL,
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_by VARCHAR(10) NULL, -- User who created the record
    updated_by VARCHAR(10) NULL  -- User who last updated the record
);

INSERT INTO users (code, username, password, first_name, middle_name, last_name, created_by, updated_by) 
    VALUES ('admin001', 'admin', md5('1234'), 'Admin', 'Middle', 'User', 'admin001', 'admin001');

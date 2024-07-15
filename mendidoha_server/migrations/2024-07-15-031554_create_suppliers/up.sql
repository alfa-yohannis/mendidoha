CREATE TABLE suppliers (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) DEFAULT LPAD(CAST(FLOOR(1 + random() * 9)::INT * 1000000000 + FLOOR(random() * 1000000000)::INT AS VARCHAR), 10, '0') NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_by VARCHAR(10) NULL,
    updated_by VARCHAR(10) NULL
);

-- Inserting three suppliers with sequential codes and fictional names
INSERT INTO suppliers (code, name, created, updated, created_by, updated_by)
VALUES 
    ('1000000001', 'Oceanic Products Ltd.', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'admin001', 'admin001'),
    ('1000000002', 'Global Solutions Inc.', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'admin001', 'admin001'),
    ('1000000003', 'Starlight Services LLC', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'admin001', 'admin001');

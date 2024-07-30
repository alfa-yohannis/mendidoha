CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    user_code VARCHAR(10) NOT NULL,
    device_id VARCHAR(36) NOT NULL,
    session_id VARCHAR(36) NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expiry_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_by VARCHAR(10) NULL, -- User who created the record
    updated_by VARCHAR(10) NULL -- User who last updated the record
    -- PRIMARY KEY (user_code, device_id)
);

INSERT INTO sessions (
    user_code,
    device_id,
    session_id,
    start_time,
    expiry_time,
    created,
    updated,
    created_by,
    updated_by
) VALUES (
    'admin001',
    'fcec045a-84ab-4aec-8544-1a6566594955',
    'dda3b4a5-62af-4b0f-8048-d2d21f753436',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP + INTERVAL '1000 days',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,
    'admin',
    'admin'
);

-- CREATE OR REPLACE FUNCTION set_expiry_time()
-- RETURNS TRIGGER AS $$
-- BEGIN
--     NEW.expiry_time := NEW.start_time + INTERVAL '2 days';
--     RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql;

-- CREATE TRIGGER set_expiry_time_trigger
-- BEFORE INSERT OR UPDATE ON sessions
-- FOR EACH ROW
-- EXECUTE FUNCTION set_expiry_time();

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    user_code VARCHAR(10) NOT NULL UNIQUE,
    device_id VARCHAR(36) NOT NULL,
    session_id VARCHAR(36) NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expiry_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_by VARCHAR(10) NULL, -- User who created the record
    updated_by VARCHAR(10) NULL -- User who last updated the record
);

CREATE OR REPLACE FUNCTION set_expiry_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.expiry_time := NEW.start_time + INTERVAL '2 days';
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_expiry_time_trigger
BEFORE INSERT OR UPDATE ON sessions
FOR EACH ROW
EXECUTE FUNCTION set_expiry_time();

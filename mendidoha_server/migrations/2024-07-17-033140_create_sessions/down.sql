-- This file should undo anything in `up.sql`
-- DROP TRIGGER IF EXISTS set_expiry_time_trigger ON sessions;
-- DROP FUNCTION IF EXISTS set_expiry_time();
DROP TABLE IF EXISTS sessions;
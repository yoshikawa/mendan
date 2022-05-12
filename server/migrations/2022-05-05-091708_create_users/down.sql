-- This file should undo anything in `up.sql`
DROP TRIGGER trg_users_updated_at ON users;
DROP INDEX users_email_idx;
DROP TABLE users;
DROP FUNCTION set_updated_at;
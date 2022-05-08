-- This file should undo anything in `up.sql`
DROP TRIGGER trg_users_updated_at ON users;
DROP UNIQUE INDEX users_email_idx ON users (email);
DROP TABLE users;
DROP FUNCTION set_updated_at;
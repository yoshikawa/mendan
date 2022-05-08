-- This file should undo anything in `up.sql`
DROP UNIQUE INDEX pre_users_token_idx ON pre_users (token);
DROP TABLE pre_users;
-- Your SQL goes here
CREATE TABLE pre_users (
  id SERIAL NOT NULL PRIMARY KEY,
  email VARCHAR(254) NOT NULL,
  token UUID NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT 't',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX pre_users_token_idx ON pre_users (token);
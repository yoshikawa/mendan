-- Your SQL goes here
CREATE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'UPDATE') THEN
        NEW.updated_at := now();
        return NEW;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  first_name VARCHAR(32) NOT NULL,
  last_name VARCHAR(32) NOT NULL,
  email VARCHAR(254) NOT NULL,
  password VARCHAR (128) NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT 'f',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX users_email_idx ON users (email);

CREATE TRIGGER trg_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE set_updated_at();
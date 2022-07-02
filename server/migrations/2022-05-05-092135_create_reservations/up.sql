-- Your SQL goes here
CREATE FUNCTION set_reservations_updated_at() RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'UPDATE') THEN
        NEW.updated_at := now();
        return NEW;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE reservations (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  start_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_reservations FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX reservations_user_id_idx ON reservations (user_id);

CREATE TRIGGER trg_reservations_updated_at BEFORE UPDATE ON reservations FOR EACH ROW EXECUTE PROCEDURE set_reservations_updated_at();
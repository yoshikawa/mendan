-- Your SQL goes here
CREATE TABLE reservations (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  start_time TIMESTAMP NOT NULL, 
  end_time TIMESTAMP NOT NULL,
  CONSTRAINT fk_reservations FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX reservations_user_id_idx ON reservations (user_id);
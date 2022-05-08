-- This file should undo anything in `up.sql`
DROP INDEX reservations_user_id_idx ON reservations (user_id);
DROP TABLE reservations;
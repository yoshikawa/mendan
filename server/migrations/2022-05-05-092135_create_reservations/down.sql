-- This file should undo anything in `up.sql`
DROP TRIGGER trg_reservations_updated_at ON reservations;
DROP INDEX reservations_user_id_idx;
DROP TABLE reservations;
DROP FUNCTION set_reservations_updated_at;
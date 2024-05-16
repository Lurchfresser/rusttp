-- This file should undo anything in `up.sql`
-- Drop the default value from the id column
ALTER TABLE overflows ALTER COLUMN id DROP DEFAULT;

-- Drop the sequence
DROP SEQUENCE overflows_id_seq;
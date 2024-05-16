-- Your SQL goes here
CREATE SEQUENCE overflows_id_seq;
ALTER TABLE overflows ALTER COLUMN id SET DEFAULT nextval('overflows_id_seq');
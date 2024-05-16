-- Your SQL goes here
ALTER TABLE "overflows" DROP COLUMN "overflowid";
ALTER TABLE "overflows" ADD COLUMN "overflow_id" INT4 NOT NULL;


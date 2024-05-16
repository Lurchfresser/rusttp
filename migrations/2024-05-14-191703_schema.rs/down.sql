-- This file should undo anything in `up.sql`
ALTER TABLE "overflows" DROP COLUMN "overflow_id";
ALTER TABLE "overflows" ADD COLUMN "overflowid" INT4 NOT NULL;


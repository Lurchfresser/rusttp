-- This file should undo anything in `up.sql`
ALTER TABLE "overflows" DROP COLUMN "published";
ALTER TABLE "overflows" ADD COLUMN "published" DATE NOT NULL;


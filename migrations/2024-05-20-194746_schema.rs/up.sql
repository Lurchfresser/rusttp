-- Your SQL goes here
ALTER TABLE "overflows" DROP COLUMN "published";
ALTER TABLE "overflows" ADD COLUMN "published" TIMESTAMP NOT NULL;


-- Your SQL goes here
CREATE TABLE "overflows"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"title" VARCHAR NOT NULL,
	"author" VARCHAR NOT NULL,
	"body" TEXT NOT NULL,
	"overflowid" INT4 NOT NULL,
	"published" DATE NOT NULL,
	"rating" INT4 NOT NULL
);


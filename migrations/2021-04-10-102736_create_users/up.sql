-- Your SQL goes here
CREATE TABLE "users" (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at TEXT NOT NULL
);

INSERT INTO "users" (name, email, created_at) VALUES ("John", "john@gmail.com", "2021-01-03T00.00.000Z");
INSERT INTO "users" (name, email, created_at) VALUES ("Diana", "diana@yahoo.com", "2021-04-10T09.10.000Z");
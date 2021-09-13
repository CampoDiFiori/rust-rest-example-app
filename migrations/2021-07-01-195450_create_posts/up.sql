-- Your SQL goes here

CREATE TABLE "posts" (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    user_id INTEGER,
    FOREIGN KEY(user_id) REFERENCES user(id) ON DELETE CASCADE
);


INSERT INTO "posts" (title, user_id) VALUES ("Why Orwell matters", 1);
INSERT INTO "posts" (title, user_id) VALUES ("Ivermectin and the crime of the century", 1);
INSERT INTO "posts" (title, user_id) VALUES ("Who was the boss?", 1);
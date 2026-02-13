-- Your SQL goes here
CREATE TABLE blogs (
  id      INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title   TEXT    NOT NULL,
  date    TEXT    NOT NULL,
  preview TEXT    NOT NULL,
  body    TEXT    NOT NULL
)
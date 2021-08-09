-- Your SQL goes here

CREATE TABLE owners (
    id         INTEGER PRIMARY KEY,
    created_at DATETIME,
    updated_at DATETIME,
    deleted_at DATETIME,
    name       TEXT NOT NULL
);
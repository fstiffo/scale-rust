-- Your SQL goes here

CREATE TABLE params (
    id                  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    valid_from          DATETIME NOT NULL,
    stairs_cleaning_fee INTEGER NOT NULL,
    cleanings_per_month INTEGER NOT NULL,
    monthly_dues_rate   INTEGER NOT NULL
);
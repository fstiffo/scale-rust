-- Your SQL goes here

-- Table: journal_entries
CREATE TABLE journal_entries (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    date        DATETIME NOT NULL,
    debit       INTEGER  NOT NULL DEFAULT (0),
    credit      INTEGER NOT NULL DEFAULT (0),
    account     INTEGER NOT NULL,
    owner_id    INTEGER,
    description TEXT,
    CONSTRAINT fk_owners_dues_payments FOREIGN KEY (
        owner_id
    )
    REFERENCES owners (id) 
);
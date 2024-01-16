-- Create images table
CREATE TABLE IF NOT EXISTS books
(
    id          INTEGER PRIMARY KEY NOT NULL,
    author      TEXT                NOT NULL,
    title       TEXT                NOT NULL
);

INSERT INTO books (author, title) VALUES ('Wolverson, Herbert', 'Hands-on Rust');
INSERT INTO books (author, title) VALUES ('Wolverson, Herbert', 'Rust Brain Teasers');

CREATE TABLE books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    author TEXT
);

INSERT INTO books (title, author) VALUES ('Hands-on Rust', 'Wolverson, Herbert');
INSERT INTO books (title, author) VALUES ('Rust Brain Teasers', 'Wolverson, Herbert');
INSERT INTO books (title, author) VALUES ('The Rust Programming Language', 'Klabnik, Steve');
INSERT INTO books (title, author) VALUES ('Programming Rust', 'Blandy, Jim');
INSERT INTO books (title, author) VALUES ('Rust in Action', 'McNamara, Tim');
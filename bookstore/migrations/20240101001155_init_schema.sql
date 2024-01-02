CREATE TABLE authors (
    id uuid NOT NULL,
    first_name varchar(64) NOT NULL,
    last_name varchar(64) NOT NULL,
    middle_name varchar(64) DEFAULT NULL,
    CONSTRAINT pk_authors PRIMARY KEY (id)
);

CREATE TABLE books (
    id uuid NOT NULL,
    title varchar(256) NOT NULL,
    author_id uuid DEFAULT NULL,
    CONSTRAINT pk_books PRIMARY KEY (id),
    CONSTRAINT fk_books_author_id_authors
        FOREIGN KEY (author_id)
            REFERENCES authors(id)
            ON DELETE SET NULL
);

CREATE TYPE user_role AS ENUM ('admin', 'user');

CREATE TABLE users (
    id uuid NOT NULL,
    first_name varchar(64) NOT NULL,
    last_name varchar(64) NOT NULL,
    middle_name varchar(64) DEFAULT NULL,
    nickname varchar(64) NOT NULL,
    hashed_password varchar(256) NOT NULL,
    date_registered timestamp DEFAULT now(),
    role user_role NOT NULL DEFAULT 'user',
    suspended boolean DEFAULT FALSE,
    CONSTRAINT pk_users PRIMARY KEY (id)
);
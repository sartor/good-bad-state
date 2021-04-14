-- Add migration script here
CREATE TABLE categories (
    id int NOT NULL GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name varchar(200) NOT NULL UNIQUE,
    created_at timestamp NOT NULL DEFAULT NOW(),
    score int NOT NULL DEFAULT 0
);

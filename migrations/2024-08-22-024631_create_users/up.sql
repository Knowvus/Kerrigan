-- Your SQL goes here
CREATE TABLE user_users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE
);
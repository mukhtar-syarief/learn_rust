-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TABLE user_detail (
    id SERIAL PRIMARY KEY,
    fullname TEXT NOT NULL,
    nickname VARCHAR(50) NOT NULL,
    address TEXT,
    age INTEGER NOT NULL,
    is_adult BOOLEAN
);
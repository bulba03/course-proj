-- Your SQL goes here
CREATE TYPE ROLE_ENUM AS ENUM ('admin', 'user', 'teacher');
CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    role ROLE_ENUM NOT NULL
);
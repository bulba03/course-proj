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

CREATE TABLE COURSES (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    teacher_id INT,
    FOREIGN KEY (teacher_id) REFERENCES Users (id)
);

-- Таблица "Lessons" (Уроки курса)
CREATE TABLE LESSONS (
    id SERIAL PRIMARY KEY,
    course_id INT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    FOREIGN KEY (course_id) REFERENCES COURSES (id)
);
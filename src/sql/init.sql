CREATE TABLE users (
    id SERIAL,
    nickname VARCHAR(50) UNIQUE,
    name VARCHAR(200),
    password TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    bio TEXT DEFAULT NULL,
    creation_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT NULL
);

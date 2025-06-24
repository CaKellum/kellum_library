CREATE TABLE games (
    id TEXT PRIMARY KEY,
    title TEXT,
    platform TEXT,
    rating TEXT,
    number_of_players INTEGER
);

CREATE TABLE movies (
    id TEXT PRIMARY KEY,
    title TEXT,
    format TEXT,
    rating TEXT
);

CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE,
    passHash TEXT NOT NULL
);

CREATE TABLE users_sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT UNIQUE NOT NULL, 
    expiry TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
)



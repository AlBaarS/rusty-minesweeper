-- Database creation
CREATE DATABASE IF NOT EXISTS RustyMinesweeperDB;
USE RustyMinesweeperDB;

-- Create a table for users
CREATE TABLE IF NOT EXISTS Users(
    user TINYTEXT PRIMARY KEY,
    games_completed INT UNSIGNED,
    best_time SMALLINT UNSIGNED
);

-- Create a table for games
CREATE TABLE IF NOT EXISTS Games(
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    user TINYTEXT NOT NULL,
    game JSON NOT NULL,
    FOREIGN KEY(user) REFERENCES Users(user)
);


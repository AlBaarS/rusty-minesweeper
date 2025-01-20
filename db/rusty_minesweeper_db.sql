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



-- Store a user in the user database
DROP PROCEDURE IF EXISTS CreateUser;

DELIMITER //

CREATE PROCEDURE CreateUser(
    user_name TINYTEXT
)

BEGIN
    INSERT INTO Users(user)
    VALUES(user_name);
    
END//

DELIMITER ;


-- Store a game under a user
DROP PROCEDURE IF EXISTS StoreGame;

DELIMITER //

CREATE PROCEDURE StoreGame(
    user_name TINYTEXT,
    game_state JSON
)

BEGIN
    INSERT INTO Games(user, game)
    VALUES(user_name, game_state)
    ON DUPLICATE KEY UPDATE
        game = game_state;

END//

DELIMITER ;


-- Fetch a game from the database if it exists
DROP PROCEDURE IF EXISTS FetchGame;

DELIMITER //

CREATE PROCEDURE FetchGame(
    user_name TINYTEXT
)

BEGIN
    SELECT game FROM Games WHERE user = user_name;

END//

DELIMITER ;

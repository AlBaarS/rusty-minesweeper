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


-- Test connection to the database
DROP PROCEDURE IF EXISTS TestConnection;

DELIMITER //

CREATE PROCEDURE TestConnection(
    test TINYTEXT
)

BEGIN
    SELECT "Connection established" IF "Test" = test;

END//

DELIMITER ;


-- Create a user to connect to when performing queries
CREATE USER IF NOT EXISTS Ýsiltýr@localhost IDENTIFIED WITH mysql_native_password BY 'qwidwh&b_hUB7&_iyubI7BGi_&_BknIU_Y8h';

GRANT USAGE ON RustyMinesweeperDB TO Ýsiltýr@localhost;

GRANT EXECUTE ON PROCEDURE CreateUser TO Ýsiltýr@localhost;
GRANT EXECUTE ON PROCEDURE StoreGame TO Ýsiltýr@localhost;
GRANT EXECUTE ON PROCEDURE FetchGame TO Ýsiltýr@localhost;
GRANT EXECUTE ON PROCEDURE TestConnection TO Ýsiltýr@localhost;

![coverage](https://git.sogyo.nl/abaars/rusty-minesweeper/badges/main/coverage.svg?job=unit-tests) in the main branch

![coverage](https://git.sogyo.nl/abaars/rusty-minesweeper/badges/develop/coverage.svg?job=unit-tests) in the development branch

![status](https://git.sogyo.nl/abaars/rusty-minesweeper/badges/main/pipeline.svg?ignore_skipped=true)

# Rusty Minesweeper
Implementation of the computer game Minesweeper, made in Rust by Alejandro Baars as his individual project at Sogyo.


# MoSCoW
## Must
* Generate a board with clickable squares
* Some squares must have mines, determined randomly
* Squares surrounding mines must indicate how many mines are in the vicinity
* The game is saved to a name/e-mail
* The game ends when mine explodes (lose) or when all non-mined squares are revealed (win)

## Should
* Place flags on suspected mines, disabling clicking
* Clicking on a square with zero mines reveals all neighbouring squares automatically (chained)
* Reveal all squares if the player loses
* Implement a timer that starts upon clicking the first tile

## Could
* Keep track of personal and global best time
* Have multiple save slots per player
* Delete saved games from GUI
* Basic animations
* Have different board sizes
* Deploy using Docker to make it platform independent

## Would
* Have different themes for the board
* Make a hexagonal implementation


# Technical goals
* Improve knowledge of front-end
* Improve knowledge of CI/CD
* Learn to work with Rust
* Make the GUI look nice (and at least use some colours)


# Personal goals
* When getting stuck, work on something else if it takes too long

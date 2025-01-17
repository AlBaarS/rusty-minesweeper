![coverage](https://git.sogyo.nl/abaars/rusty-minesweeper/badges/main/coverage.svg?job=unit-tests) in the main branch

![coverage](https://git.sogyo.nl/abaars/rusty-minesweeper/badges/develop/coverage.svg?job=unit-tests) in the development branch

![status](https://git.sogyo.nl/abaars/rusty-minesweeper/badges/main/pipeline.svg?ignore_skipped=true)

# Rusty Minesweeper
Implementation of the computer game Minesweeper, made in Rust by Alejandro Baars as his individual project at Sogyo.


# How to install:
## Download Rusty Minesweeper
You can download a [release](https://git.sogyo.nl/abaars/rusty-minesweeper/-/releases/permalink/latest) or clone the source code using git:

`git clone git@git.sogyo.nl:abaars/rusty-minesweeper.git`

Alternatively, you can download the source code using wget:

`wget https://git.sogyo.nl/abaars/rusty-minesweeper.git`

## Install dependencies 

First and foremost, make sure that [Rust](https://www.rust-lang.org/tools/install) is installed on your system. You can check using `rustup --help` on your CLI. Next, install [nvm and Node.js](https://nodejs.org/en/download).

With Rust, nvm, and Node.js installed, you can install the project dependencies with the following commands:

```
cd rusty-minesweeper/
npm install
npm update
```

Finally, open two CLI windows or `screen`s. Navigate to the project root `rusty-minesweeper/` in one and to `rusty-minesweeper/backend/` in the other.

In `rusty-minesweeper/backend/`, execute the command `cargo run`.

In `rusty-minesweeper/`, execute the command `npm run dev`.

Open your web browser of choice and navigate to <http://localhost:3000/>. The game will open on its starting page and you are ready to play!

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

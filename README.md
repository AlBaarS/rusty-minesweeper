![coverage](https://git.sogyo.nl/abaars/rusty-minesweeper/badges/main/coverage.svg?job=unit-tests) ![status](https://git.sogyo.nl/abaars/rusty-minesweeper/badges/main/pipeline.svg?ignore_skipped=true)

# Rusty Minesweeper
Implementation of the computer game Minesweeper, made in Rust by Alejandro Baars as his individual project at Sogyo.


# How to install:
## Download Rusty Minesweeper
You can download a docker image or the source code as a [release](https://git.sogyo.nl/abaars/rusty-minesweeper/-/releases/permalink/latest), or clone the repository using git:

`git clone git@git.sogyo.nl:abaars/rusty-minesweeper.git`

Or using wget:

`wget https://git.sogyo.nl/abaars/rusty-minesweeper.git`

## Run using the docker image

The easiest way to play Rusty Minesweeper is using the provided docker image. Make sure you have [Docker](https://docs.docker.com/engine/install/) installed on your computer before using this programme. Once you've downloaded Rusty Minesweeper, navigate to the directory with the rusty-minesweeper_{version}.tar.gz file and use the following commands in your terminal:

```
tar -zxf rusty-minesweeper_{version}.tar.gz         # unpacks the tarball
docker load -i rusty-minesweeper.tar                # loads the file onto docker
docker compose up -d                                # runs the programme and detaches it from your terminal
```

Once it's running, open your web browser of choice and navigate to <http://localhost:3000/>. The game will open on its starting page and you are ready to play. You can close the terminal while the programme is active if you wish.

To terminate, simply type `docker compose down` while in your terminal window that is running the programme.

## Run from source

### Install dependencies

Install the following:
* [Rust](https://www.rust-lang.org/tools/install)
* [nvm and Node.js](https://nodejs.org/en/download)
* [MongoDB](https://www.mongodb.com/docs/manual/administration/install-community/)

Then you can install the project dependencies with the following commands:

```
cd rusty-minesweeper/   # navigate into the rusty-minesweeper directory
npm install             # install dependencies
npm update              # update the state of the dependencies
```

### Starting and terminating the programme

To start the programme, simply use the provided `.init.sh` script as follows:
```
bash .init.sh           # this script takes care of starting the programme.
```

The script assumes the user has access to the systemctl command. Use 'sudo' if you get a permission error. The first time you do this it might take a while for everything to build. Once that's done, open your web browser of choice and navigate to <http://localhost:3000/>. The game will open on its starting page and you are ready to play.

To terminate, simply do `Ctrl + C` __twice__ while in your terminal window that is running the programme.

# Deploy using Ansible

This feature was sadly not completed before I had to move on to my new employer, but it is very close. Sadly, the final step doesn't seem to be possible without a major revision.

Currently, Ansible puts all the required files and installs the required programmes onto a virtual machine and starts the docker container. It gets the authentication and addresses for these VMs dynamically from an Infisical vault. But sadly the docker container doesn't work properly because the IP addresses need to be passed dynamically at build time. The Ansible step only triggers *after* the build has been made. I suspect that the solution would be to add a new script (with Ansible or something else) to retrieve the IP addresses from the vault before build time, build a container for each VM, and then send each individual container off to their respective machine. But sadly, I do not have the time to implement that.

To find out how the machines are created, I direct you to take a look at my [terraform](https://git.sogyo.nl/abaars/mancala-terraform) project ([github link](https://github.com/AlBaarS/terraform_experimentation) for non-sogyo users).

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
* Difficulty selector
* Deploy using Docker to make it platform independent
* Keep track of personal and global best time
* Have different board sizes


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

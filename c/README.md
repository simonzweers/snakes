# Snake

This is a simple snake game that can be played in the terminal.

## Prerequisites
The project needs ncurses to be installed.  
On debian/ubuntu based distribitions:

```bash
sudo apt install libncurses-dev libncurses6 -y
```

## Installation

```bash
git clone https://github.com/simonzweers/csnake.git
cd csnake
make

# Optional: for installing on the system
sudo make install
```

## To play

To play:
```bash
./snake # To run without installing on system
```

When snake is installed on the system:
```bash
snake
```
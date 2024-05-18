package main

import (
	"log"

	"github.com/rthornton128/goncurses"
	"github.com/simonzweers/snakes/go/snakego/display"
	"github.com/simonzweers/snakes/go/snakego/snake"
)

func main() {
	snake.SnakeInit()
	stdscr, err := goncurses.Init()
	if err != nil {
		log.Fatal("init:", err)
	}
	defer goncurses.End()
	goncurses.Cursor(0)   // hide cursor
	goncurses.Echo(false) // turn echoing of typed characters off
	stdscr.Keypad(true)   // allow keypad input

	display.DisplayField(stdscr)
	stdscr.Refresh()
	stdscr.GetChar()
}

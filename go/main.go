package main

import (
	"log"
	"os"
	"os/signal"
	"time"

	"github.com/rthornton128/goncurses"
	"github.com/simonzweers/snakes/go/snakego/display"
)

func main() {
	// Init ncurses
	stdscr, err := goncurses.Init()
	if err != nil {
		log.Fatal("init:", err)
	}
	defer goncurses.End()
	goncurses.Cursor(0)   // hide cursor
	goncurses.Echo(false) // turn echoing of typed characters off
	stdscr.Keypad(true)   // allow keypad input

	// Catch Ctrl-C
	var gameloop bool = true
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	go func() {
		<-c
		gameloop = false
	}()

	// Init snake logic

	for gameloop {
		time.Sleep(100 * 1000)
		display.DisplayField(stdscr)
		stdscr.Refresh()
	}
}

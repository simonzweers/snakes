package main

import (
	"log"
	"os"
	"os/signal"

	"github.com/gdamore/tcell/v2"
)

func main() {
	// Init ncurses
	stdscr, err := tcell.NewScreen()
	if err != nil {
		log.Fatal("NewScreen:", err)
	}
	if err := stdscr.Init(); err != nil {
		log.Fatal("Init:", err)
	}

	defer stdscr.Fini()

	// Catch Ctrl-C
	var gameloop bool = true
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	go func() {
		<-c
		gameloop = false
	}()

	for {

	}
}

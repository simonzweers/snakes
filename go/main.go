package main

import (
	"log"
	"os"
	"os/signal"
	"time"

	"github.com/gdamore/tcell/v2"

	"github.com/simonzweers/snakes/go/snakego/display"
	"github.com/simonzweers/snakes/go/snakego/snake"
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

	// Init snake logic
	snakeContext := snake.NewSnake()

	for gameloop {
		display.DisplayField(stdscr)
		display.DisplaySnake(stdscr, snakeContext)
		stdscr.Show()
		time.Sleep(1000 * 1000)
	}
}

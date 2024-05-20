package main

import (
	"log"

	"github.com/gdamore/tcell/v2"
	"github.com/simonzweers/snakes/go/snakego/gamecontext"
)

func main() {
	// Init tcell
	stdscr, err := tcell.NewScreen()
	if err != nil {
		log.Fatal("NewScreen:", err)
	}
	if err := stdscr.Init(); err != nil {
		log.Fatal("Init:", err)
	}

	defer stdscr.Fini()

	// Catch Keyboard input
	c := make(chan int, 1)

	gamecontext := gamecontext.NewGameContext(stdscr)

	go gamecontext.StartGame(c)
	gamecontext.HandleKeyboardInput(c)
}

package gamecontext

import (
	"github.com/gdamore/tcell/v2"
)

const (
	FIELD_HEIGHT = 20
	FIELD_WIDTH  = 30
)

type GameContext struct {
	Snake  Snake
	screen tcell.Screen
}

func NewGameContext(newScreen tcell.Screen) *GameContext {
	gamecontext := new(GameContext)
	gamecontext.Snake = newSnake()
	gamecontext.screen = newScreen
	return gamecontext
}

func (gamecontext *GameContext) HandleKeyboardInput(input chan int) {
	defer close(input)

	for {
		event := gamecontext.screen.PollEvent()
		if event == nil {
			break
		}
		switch eventType := event.(type) {
		case *tcell.EventKey:
			if eventType.Key() == tcell.KeyEscape || eventType.Key() == tcell.KeyCtrlC {
				gamecontext.screen.Fini()
			}
		}
	}
}

package gamecontext

import (
	"sync"
	"time"

	"github.com/gdamore/tcell/v2"
)

const (
	FIELD_HEIGHT = 20
	FIELD_WIDTH  = 30
)

type GameContext struct {
	Snake      Snake
	screen     tcell.Screen
	gameActive bool
	mutext     sync.Mutex
}

func NewGameContext(newScreen tcell.Screen) *GameContext {
	gamecontext := new(GameContext)
	gamecontext.Snake = newSnake()
	gamecontext.screen = newScreen
	return gamecontext
}

func (gc *GameContext) HandleKeyboardInput(input chan int) {
	defer close(input)

	for {
		event := gc.screen.PollEvent()
		if event == nil {
			break
		}
		switch eventType := event.(type) {
		case *tcell.EventKey:
			key := eventType.Key()
			if key == tcell.KeyEscape || key == tcell.KeyCtrlC {
				gc.screen.Fini()
			}

			switch key {
			case tcell.KeyUp:
				input <- int(UP)
			case tcell.KeyDown:
				input <- int(DOWN)
			case tcell.KeyLeft:
				input <- int(LEFT)
			case tcell.KeyRight:
				input <- int(RIGHT)
			}
		}
	}
}

func (gc *GameContext) StartGame(input chan int) {
	ticker := time.NewTicker(100 * time.Millisecond)
	for {
		<-ticker.C

		dir := <-input
		switch dir {
		case int(UP):
			gc.screen.SetContent(3, 3, tcell.RuneUArrow, nil, tcell.StyleDefault)
		case int(DOWN):
			gc.screen.SetContent(3, 3, tcell.RuneDArrow, nil, tcell.StyleDefault)
		case int(LEFT):
			gc.screen.SetContent(3, 3, tcell.RuneLArrow, nil, tcell.StyleDefault)
		case int(RIGHT):
			gc.screen.SetContent(3, 3, tcell.RuneRArrow, nil, tcell.StyleDefault)
		}
		gc.screen.Show()
	}
}

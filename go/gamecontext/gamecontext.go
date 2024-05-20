package gamecontext

import (
	"sync"
	"time"

	"github.com/gdamore/tcell/v2"
)

const (
	FIELD_SIZE_Y = 20
	FIELD_SIZE_X = 30
)

type GameContext struct {
	Snake      Snake
	screen     tcell.Screen
	gameActive bool
	mutex      sync.Mutex
}

func NewGameContext(newScreen tcell.Screen) *GameContext {
	gamecontext := new(GameContext)
	gamecontext.Snake = newSnake()
	gamecontext.screen = newScreen
	gamecontext.gameActive = true
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
				gc.gameActive = false
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
	frame := 0

	go func() {
		for {
			dir := <-input
			gc.parseDirection(Direction(dir))
		}
	}()

	for gc.gameActive {
		<-ticker.C
		gc.screen.Clear()
		gc.Snake.propogate()
		gc.drawField()
		gc.drawSnake()
		gc.screen.Show()
		frame++
	}
	gc.screen.Fini()
}

func (gc *GameContext) parseDirection(dir Direction) {
	gc.mutex.Lock()
	switch dir {
	case UP:
		gc.screen.SetContent(3, FIELD_SIZE_Y+3, tcell.RuneUArrow, nil, tcell.StyleDefault)
	case DOWN:
		gc.screen.SetContent(3, FIELD_SIZE_Y+3, tcell.RuneDArrow, nil, tcell.StyleDefault)
	case LEFT:
		gc.screen.SetContent(3, FIELD_SIZE_Y+3, tcell.RuneLArrow, nil, tcell.StyleDefault)
	case RIGHT:
		gc.screen.SetContent(3, FIELD_SIZE_Y+3, tcell.RuneRArrow, nil, tcell.StyleDefault)
	}
	gc.Snake.dir = dir
	gc.mutex.Unlock()
}

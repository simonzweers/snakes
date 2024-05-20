package display

import (
	"github.com/gdamore/tcell/v2"
	"github.com/simonzweers/snakes/go/snakego/snake"
)

func DisplayField(win tcell.Screen) {
	for y := 0; y <= snake.FIELD_SIZE_Y; y++ {
		for x := 0; x <= snake.FIELD_SIZE_X; x++ {
			if x == snake.FIELD_SIZE_X {
				win.SetContent(x, y, tcell.RuneVLine, nil, tcell.StyleDefault)
			}
			if y == snake.FIELD_SIZE_Y {
				win.SetContent(x, y, tcell.RuneHLine, nil, tcell.StyleDefault)
			}
			if x == snake.FIELD_SIZE_X && y == snake.FIELD_SIZE_Y {
				win.SetContent(x, y, tcell.RuneLRCorner, nil, tcell.StyleDefault)
				// win.MovePrint(y, x*2, "+ ")
			}
		}
	}
}

func DisplaySnake(win tcell.Screen, snake snake.Snake) {
	// win.MovePrint(snake.Pos.Y, snake.Pos.X*2, "##")
}

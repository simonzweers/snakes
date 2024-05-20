package gamecontext

import "github.com/gdamore/tcell/v2"

func (gc *GameContext) drawField() {
	for y := 0; y <= FIELD_SIZE_Y; y++ {
		for x := 0; x <= FIELD_SIZE_X; x++ {
			if x == FIELD_SIZE_X && y == FIELD_SIZE_Y {
				gc.screen.SetContent(x*2, y, tcell.RuneLRCorner, nil, tcell.StyleDefault)
			} else if x == FIELD_SIZE_X {
				gc.screen.SetContent(x*2, y, tcell.RuneVLine, nil, tcell.StyleDefault)
			} else if y == FIELD_SIZE_Y {
				gc.screen.SetContent(x*2, y, tcell.RuneHLine, nil, tcell.StyleDefault)
				gc.screen.SetContent(x*2+1, y, tcell.RuneHLine, nil, tcell.StyleDefault)
			}
		}
	}
}

func (gc *GameContext) drawSnake() {
	gc.screen.SetContent(
		gc.Snake.headPosition.X*2, gc.Snake.headPosition.Y,
		tcell.RuneBlock,
		nil, tcell.StyleDefault,
	)
	gc.screen.SetContent(
		gc.Snake.headPosition.X*2+1, gc.Snake.headPosition.Y,
		tcell.RuneBlock,
		nil, tcell.StyleDefault,
	)
}

func drawText(s tcell.Screen, x1, y1 int, text string) {
	row := y1
	col := x1
	for _, r := range text {
		s.SetContent(col, row, r, nil, tcell.StyleDefault)
		col++
	}
}

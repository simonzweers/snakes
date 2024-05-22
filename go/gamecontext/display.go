package gamecontext

import (
	"fmt"

	"github.com/gdamore/tcell/v2"
)

func (gc *GameContext) drawField() {
	for y := 0; y <= FIELD_SIZE_Y; y++ {
		for x := 0; x <= FIELD_SIZE_X; x++ {
			if x == FIELD_SIZE_X && y == FIELD_SIZE_Y {
				gc.screen.SetContent(x*2, y, '+', nil, tcell.StyleDefault)
			} else if x == FIELD_SIZE_X {
				gc.screen.SetContent(x*2, y, '|', nil, tcell.StyleDefault)
			} else if y == FIELD_SIZE_Y {
				gc.screen.SetContent(x*2, y, '-', nil, tcell.StyleDefault)
				gc.screen.SetContent(x*2+1, y, '-', nil, tcell.StyleDefault)
			}
		}
	}
}

func (gc *GameContext) drawSnake() {
	cursor := gc.Snake.head
	i := 0
	for !(cursor == nil) {
		gc.screen.SetContent(
			cursor.pos.X*2, cursor.pos.Y,
			'[',
			nil, tcell.StyleDefault,
		)
		gc.screen.SetContent(
			cursor.pos.X*2+1, cursor.pos.Y,
			']',
			nil, tcell.StyleDefault,
		)
		// display all the nodes on screen
		drawText(gc.screen, FIELD_SIZE_X+3, FIELD_SIZE_Y+3+i, fmt.Sprintf("Snek part X: %d | Y: %d", cursor.pos.X, cursor.pos.Y))
		cursor = cursor.next
		i++
	}
}

func drawText(s tcell.Screen, x1, y1 int, text string) {
	row := y1
	col := x1
	for _, r := range text {
		s.SetContent(col, row, r, nil, tcell.StyleDefault)
		col++
	}
}

func (gc *GameContext) drawFood() {
	gc.screen.SetContent(
		gc.food.X*2, gc.food.Y,
		'(',
		nil, tcell.StyleDefault,
	)
	gc.screen.SetContent(
		gc.food.X*2+1, gc.food.Y,
		')',
		nil, tcell.StyleDefault,
	)
}

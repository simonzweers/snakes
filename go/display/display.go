package display

import (
	"log"

	"github.com/rthornton128/goncurses"
	"github.com/simonzweers/snakes/go/snakego/snake"
)

func InitDisplay() {
	win, err := goncurses.Init()
	if err != nil {
		log.Fatal("init:", err)
	}
	var hello goncurses.Key = win.GetChar()
	win.Println(hello)
	defer goncurses.End()
}

func DisplayField(win *goncurses.Window) {
	for y := 0; y <= snake.FIELD_SIZE_Y; y++ {
		for x := 0; x <= snake.FIELD_SIZE_X; x++ {
			if x == snake.FIELD_SIZE_X {
				win.MovePrint(y, x*2, "| ")
			}
			if y == snake.FIELD_SIZE_Y {
				win.MovePrint(y, x*2, "--")
			}
			if x == snake.FIELD_SIZE_X && y == snake.FIELD_SIZE_Y {
				win.MovePrint(y, x*2, "+ ")
			}
		}
	}
}

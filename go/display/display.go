package display

import (
	"log"

	"github.com/rthornton128/goncurses"
)

func InitDisplay() {
	src, err := goncurses.Init()
	if err != nil {
		log.Fatal("init:", err)
	}
	var hello goncurses.Key = src.GetChar()
	src.Println(hello)
	defer goncurses.End()
}

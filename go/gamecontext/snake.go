package gamecontext

type Direction int

const (
	UP Direction = 1 + iota
	DOWN
	LEFT
	RIGHT
)

type Vector2 struct {
	X int
	Y int
}

type Snake struct {
	headPosition Vector2
	dir          Direction
}

func newSnake() (snake Snake) {
	snake.dir = LEFT
	snake.headPosition = Vector2{
		X: FIELD_WIDTH / 2,
		Y: FIELD_HEIGHT / 2,
	}
	return
}

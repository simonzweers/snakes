package snake

const (
	FIELD_SIZE_X = 30
	FIELD_SIZE_Y = 20
)

type Vertex struct {
	X int
	Y int
}

type Snake struct {
	Pos Vertex
	Dir Vertex
	q   []int
}

func NewSnake() (snake Snake) {
	snake.Pos.X = FIELD_SIZE_X / 2
	snake.Pos.Y = FIELD_SIZE_Y / 2
	snake.Dir.X = 0
	snake.Dir.Y = 0
	return
}

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

type Node struct {
	pos  Vector2
	next *Node
}

type Snake struct {
	headPosition Vector2
	dir          Direction
	head         *Node
	len          int
}

func (s *Snake) addBodypart() {
	newHead := new(Node)
	newHead.pos.X = s.headPosition.X
	newHead.pos.Y = s.headPosition.Y
	newHead.next = s.head
	s.head = newHead
	s.len++
}

func (s *Snake) removeTail() {
	cursor := s.head
	cursorPrev := cursor
	if cursor.next == nil {
		s.head = nil
	} else {
		for cursor.next != nil {
			cursorPrev = cursor
			cursor = cursor.next
		}
		cursorPrev.next = nil
	}
	cursor = nil
	s.len--
}

func (s *Snake) move() {
	s.removeTail()

	s.addBodypart()
}

func newSnake() (snake Snake) {
	snake.dir = LEFT
	snake.headPosition = Vector2{
		X: FIELD_SIZE_X / 2,
		Y: FIELD_SIZE_Y / 2,
	}
	snake.addBodypart()
	return
}

func (s *Snake) propogate() {
	switch s.dir {
	case UP:
		s.headPosition.Y -= 1
	case DOWN:
		s.headPosition.Y += 1
	case LEFT:
		s.headPosition.X -= 1
	case RIGHT:
		s.headPosition.X += 1
	}
}

func (s *Snake) isInField() bool {
	return (s.headPosition.X >= 0) && (s.headPosition.X < FIELD_SIZE_X) && (s.headPosition.Y >= 0) && (s.headPosition.Y < FIELD_SIZE_Y)
}

func (s *Snake) isColliding() bool {
	cursor := s.head.next
	if cursor == nil {
		return false
	}
	for cursor != nil {
		if (cursor.pos.X == s.headPosition.X) && (cursor.pos.Y == s.headPosition.Y) {
			return true
		}
		cursor = cursor.next
	}
	return false
}

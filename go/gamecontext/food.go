package gamecontext

import "math/rand"

type Food Vector2

func (f *Food) newFood() {
	f.X = rand.Intn(FIELD_SIZE_X)
	f.Y = rand.Intn(FIELD_SIZE_Y)
}

package main

type Space int

const (
	Empty Space = iota
	A
	B
	C
	D
)

type State struct {
	spaces [19]Space
	cost   int
}

func (s State) neighbors() []State {

}

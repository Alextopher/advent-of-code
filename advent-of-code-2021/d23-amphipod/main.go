package main

import (
	"container/heap"
	"fmt"
	"strings"
	"time"

	"golang.org/x/exp/constraints"
)

// I'm thinking dijkstra's algorithm will work here
// Each Vertex is state (position of all the amphipods) and the edges respesent the cost of moving between states
// So the todo list is
//
// Represent the state
// Generate the next states and calculate the cost of moving to each of them
// Run dijkstra's algorithm
// (maybe A* will be needed)

func Abs[T constraints.Signed](x int) int {
	if x < 0 {
		return -x
	}
	return x
}

var amphipodEnergy = map[byte]int{
	'A': 1,
	'B': 10,
	'C': 100,
	'D': 1000,
}

var hallway = []Position{{1, 1}, {2, 1}, {4, 1}, {6, 1}, {8, 1}, {10, 1}, {11, 1}}

const part1 string = `#############
#...........#
###B#B#C#D###
  #D#C#A#A#  
  #########  `

const part1goal string = `#############
#...........#
###A#B#C#D###
  #A#B#C#D#  
  #########  `

var amphipodHomes1 = map[byte][]Position{
	'A': {{3, 3}, {3, 2}},
	'B': {{5, 3}, {5, 2}},
	'C': {{7, 3}, {7, 2}},
	'D': {{9, 3}, {9, 2}},
}

const part2 string = `#############
#...........#
###B#B#C#D###
  #D#C#B#A#  
  #D#B#A#C#  
  #D#C#A#A#  
  #########  `

const part2goal string = `#############
#...........#
###A#B#C#D###
  #A#B#C#D#  
  #A#B#C#D#  
  #A#B#C#D#  
  #########  `

var amphipodHomes2 = map[byte][]Position{
	'A': {{3, 5}, {3, 4}, {3, 3}, {3, 2}},
	'B': {{5, 5}, {5, 4}, {5, 3}, {5, 2}},
	'C': {{7, 5}, {7, 4}, {7, 3}, {7, 2}},
	'D': {{9, 5}, {9, 4}, {9, 3}, {9, 2}},
}

// Calculates the cost of moving from one position to another
// not to be confused with the bigDijkstra function
func smallDijkstra(state State, start Position) map[Position]int {
	// queue := PriorityQueue{}
	queue := MinHeap[Position]{}

	distances := make(map[Position]int)
	prev := make(map[Position]Position)
	for x := 0; x < state.width; x++ {
		for y := 0; y < state.height; y++ {
			p := Position{x, y}
			if p == start {
				distances[p] = 0
			} else {
				distances[p] = 255
			}

			prev[p] = Position{-1, -1}

			item := &HeapItem[Position]{
				value:    p,
				priority: distances[p],
			}

			heap.Push(&queue, item)
		}
	}

	for queue.Len() > 0 {
		// calculate neighbors
		item := queue.Pop().(*HeapItem[Position])
		p3 := item.value

		// can only move 1 space at a time
		for _, p4 := range []Position{
			{p3.x - 1, p3.y},
			{p3.x + 1, p3.y},
			{p3.x, p3.y - 1},
			{p3.x, p3.y + 1},
		} {
			if p4.x < 0 || p4.x >= state.width || p4.y < 0 || p4.y >= state.height {
				continue
			}

			if state.locate(p4) != '.' {
				continue
			}

			alt := distances[p3] + 1
			if alt < distances[p4] {
				distances[p4] = alt
				prev[p4] = p3

				queue.Push(&HeapItem[Position]{
					value:    p4,
					priority: alt,
				})
			}
		}
	}

	return distances
}

// State is a grid maintained by a 1d string
type State struct {
	width, height int
	grid          string
}

func (s *State) set(amphipod byte, pos Position) {
	str := []byte(s.grid)
	str[pos.y*s.width+pos.x] = amphipod
	s.grid = string(str)
}

func NewState(input string) State {
	// split by newline
	lines := strings.Split(input, "\n")
	// get the width and height
	width := len(lines[0])
	height := len(lines)

	// rejoin the lines into a single string
	grid := strings.Join(lines, "")

	return State{width, height, grid}
}

func (s State) NextStates() (states []State, costs []int) {
	states = make([]State, 0)
	costs = make([]int, 0)

	// loop through the state to find each amphipod
	for y := 0; y < s.height; y++ {
		for x := 0; x < s.width; x++ {
			// if the current space is an amphipod
			p := Position{x, y}
			amphipod := s.locate(p)
			if amphipod == 'A' || amphipod == 'B' || amphipod == 'C' || amphipod == 'D' {
				// calculate the cost of moving to all other spaces
				distances := smallDijkstra(s, p)

				if y == 1 {
					// from the hallway it can only move into it's room
					for _, homePos := range amphipodHomes[amphipod] {
						// if the space is empty move to it
						if s.locate(homePos) == '.' {
							// calculate the distance to the hallway
							distance := distances[homePos]
							if distance >= 255 {
								break
							}

							// create a new state
							state := State{s.width, s.height, strings.Clone(s.grid)}

							// set the amphipod to the hallway
							state.set(amphipod, homePos)

							// remove the current amphipod
							state.set('.', p)

							states = append(states, state)
							costs = append(costs, distance*amphipodEnergy[amphipod])
							break
						} else if s.locate(homePos) != amphipod {
							// if the space is occupied by another amphipod break
							break
						}
					}
				} else {
					// check if the amphipod is in it's home room and that all of the squares beneath it are of the same type
					isHome := false
					for _, homePos := range amphipodHomes[amphipod] {
						if s.locate(homePos) != amphipod {
							break
						}

						if p == homePos {
							isHome = true
						}
					}
					if isHome {
						continue
					}

					// move into the hallway
					for _, hallwayPos := range hallway {
						// calculate the distance to the hallway
						distance := distances[hallwayPos]
						if distance >= 255 {
							continue
						}

						// create a new state
						state := State{s.width, s.height, strings.Clone(s.grid)}

						// set the amphipod to the hallway
						state.set(amphipod, hallwayPos)

						// remove the current amphipod
						state.set('.', p)

						states = append(states, state)
						costs = append(costs, distance*amphipodEnergy[amphipod])
					}
				}
			} else {
				continue
			}
		}
	}

	return
}

func (s State) String() string {
	result := ""
	for i := 0; i < s.height; i++ {
		result += s.grid[i*s.width : (i+1)*s.width]
		result += "\n"
	}
	return result
}

type Position struct {
	x, y int
}

func (s State) locate(pos Position) byte {
	return s.grid[pos.y*s.width+pos.x]
}

type BigDijkstra struct{}

func (b BigDijkstra) Neighbors(state State) ([]State, []int) {
	return state.NextStates()
}

func (b BigDijkstra) Estimate(state State, goal State) int {
	// we need an underestimate so I'm going to define the estimate based how many amphipods are not yet "home"
	// scaled by the energy of moving that amphipod hozizontally to it's home room
	estimate := 0
	for y := 0; y < state.height; y++ {
		for x := 0; x < state.width; x++ {
			p := Position{x, y}
			amphipod := state.locate(p)
			if amphipod == 'A' || amphipod == 'B' || amphipod == 'C' || amphipod == 'D' {
				isHome := false

				// check if the amphipod is in it's home room
				for _, homePos := range amphipodHomes[amphipod] {
					if homePos == p {
						isHome = true
						break
					}
				}

				if !isHome {
					// estimate based on the horizontal distance to the home room
					estimate += amphipodEnergy[amphipod] * Abs[int](amphipodHomes[amphipod][0].x-x)
				}
			}
		}
	}

	return estimate
}

var amphipodHomes map[byte][]Position

func main() {
	// part 1
	t := time.Now()
	start := NewState(part1)
	target := NewState(part1goal)
	amphipodHomes = amphipodHomes1

	DijkstraSearch[State](start, target, BigDijkstra{})
	fmt.Println("Solved in", time.Since(t))

	// part 2
	t = time.Now()
	start = NewState(part2)
	target = NewState(part2goal)
	amphipodHomes = amphipodHomes2

	DijkstraSearch[State](start, target, BigDijkstra{})
	fmt.Println("Solved in", time.Since(t))
}

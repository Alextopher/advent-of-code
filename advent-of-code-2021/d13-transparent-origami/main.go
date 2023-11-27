package main

import (
	"fmt"
	"os"
)

type Point struct {
	x, y int
}

func printPoints(points map[Point]struct{}) {
	// find maximum x and y
	maxX, maxY := 0, 0
	for point := range points {
		if point.x > maxX {
			maxX = point.x
		}
		if point.y > maxY {
			maxY = point.y
		}
	}

	for y := 0; y <= maxY; y++ {
		for x := 0; x <= maxX; x++ {
			if _, ok := points[Point{x, y}]; ok {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

func main() {
	// open file
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	// A set of lines are the points on the non-folded piece of paper
	// There is no container/set package right now so I use a map of empty structs
	points := make(map[Point]struct{})
	nextPoints := make(map[Point]struct{})

	for {
		var point Point
		n, _ := fmt.Fscanf(file, "%d,%d", &point.x, &point.y)
		if n == 0 {
			break
		}

		points[point] = struct{}{}
	}

	// Now to run the rules
	i := 0
	for {
		var direction rune
		var position int

		// fold along y=7
		_, err := fmt.Fscanf(file, "fold along %c=%d", &direction, &position)
		if err != nil {
			break
		}

		if i == 1 {
			fmt.Println(len(points))
		}

		// Update points
		for point := range points {
			if direction == 'x' {
				if point.x > position {
					nextPoints[Point{x: 2*position - point.x, y: point.y}] = struct{}{}
				} else {
					nextPoints[Point{x: point.x, y: point.y}] = struct{}{}
				}
			} else {
				if point.y > position {
					nextPoints[Point{x: point.x, y: 2*position - point.y}] = struct{}{}
				} else {
					nextPoints[Point{x: point.x, y: point.y}] = struct{}{}
				}
			}
		}

		// swap points
		points, nextPoints = nextPoints, make(map[Point]struct{})
		i++
	}

	printPoints(points)
}

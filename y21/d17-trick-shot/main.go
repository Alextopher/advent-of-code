package main

import (
	"fmt"
	"os"
)

// simulate the ball's flight along the y axis
func simulateY(vy, ymin, ymax int) (bool, int) {
	// y velocity decreases by 1 every second
	// return true if at any point the ball is in the target area
	var apex, y int
	for {
		if y > apex {
			apex = y
		}

		if y < ymin {
			return false, apex
		}

		if ymin <= y && y <= ymax {
			return true, apex
		}

		y += vy
		vy--
	}
}

func simulate(vx, vy, xmin, xmax, ymin, ymax int) bool {
	// x velocity decreases by 1 every second until it reaches 0
	// y velocity increases by 1 every second
	// return true if at any point the ball is in the target area
	var x, y int
	for {
		if x > xmax || y < ymin {
			return false
		}

		if xmin <= x && x <= xmax && ymin <= y && y <= ymax {
			return true
		}

		x += vx
		y += vy
		if vx > 0 {
			vx--
		}
		vy -= 1
	}
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// target area: x=207..263, y=-115..-63
	var xmin, xmax, ymin, ymax int
	fmt.Fscanf(file, "target area: x=%d..%d, y=%d..%d\n", &xmin, &xmax, &ymin, &ymax)

	// best vy is the initial velocity that produces the highest apex while still hitting the target area
	var bestvy, bestApex int
	for i := 0; i < 500; i++ {
		if ok, apex := simulateY(i, ymin, ymax); ok {
			if apex > bestApex {
				bestApex = apex
				bestvy = i
			}
		}
	}

	// find all initial velocities that produce a ball that hits the target area
	count := 0
	for vx := 0; vx < xmax+1; vx++ {
		for vy := ymin; vy <= bestvy; vy++ {
			if simulate(vx, vy, xmin, xmax, ymin, ymax) {
				count++
			}
		}
	}

	fmt.Println(bestApex)
	fmt.Println(count)
}

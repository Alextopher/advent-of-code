package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

type position struct {
	x, y int
}

func dijkstra(grid [][]byte, start position, end position, size int) int {
	queue := PriorityQueue{}

	// create distance and previous grids
	type distprev struct {
		dist int
		prev position
	}

	dp := make([][]distprev, size)
	for i := 0; i < size; i++ {
		dp[i] = make([]distprev, size)
		for j := 0; j < size; j++ {
			if i == 0 && j == 0 {
				dp[i][j].dist = 0
			} else {
				// initialize distance to infinity
				dp[i][j].dist = math.MaxInt32
			}

			// initialize previous to nil
			dp[i][j].prev = position{-1, -1}

			queue.Push(&Item{value: position{i, j}, priority: dp[i][j].dist})
		}
	}

	// while queue is not empty
	for queue.Len() > 0 {
		// get the next item
		item := queue.Pop().(*Item)

		// fmt.Println("Item:", item.value, item.priority)

		// calculate neighbors
		neighbors := make([]position, 0)
		if item.value.x > 0 {
			neighbors = append(neighbors, position{item.value.x - 1, item.value.y})
		}
		if item.value.x < size-1 {
			neighbors = append(neighbors, position{item.value.x + 1, item.value.y})
		}
		if item.value.y > 0 {
			neighbors = append(neighbors, position{item.value.x, item.value.y - 1})
		}
		if item.value.y < size-1 {
			neighbors = append(neighbors, position{item.value.x, item.value.y + 1})
		}

		// for each neighbor
		for _, neighbor := range neighbors {
			alt := dp[item.value.y][item.value.x].dist + int(grid[neighbor.y][neighbor.x])
			if alt < dp[neighbor.y][neighbor.x].dist {
				dp[neighbor.y][neighbor.x].dist = alt
				dp[neighbor.y][neighbor.x].prev = item.value

				queue.Push(&Item{value: neighbor, priority: alt})
			}
		}
	}

	return dp[size-1][size-1].dist
}

func part1(grid [][]byte, size int) int {
	return dijkstra(grid, position{0, 0}, position{size - 1, size - 1}, size)
}

func part2(grid [][]byte, size int) int {
	// Create the larger grid
	larger := make([][]byte, size*5)
	for i := 0; i < size*5; i++ {
		larger[i] = make([]byte, size*5)
	}

	// Fill the larger grid with the inputs
	for y := 0; y < 5*size; y++ {
		for x := 0; x < 5*size; x++ {
			// get the position in the original
			xpos := x % size
			ypos := y % size

			// get the "grid" position
			xgrid := x / size
			ygrid := y / size

			// get the value
			larger[y][x] = grid[ypos][xpos] + byte(xgrid) + byte(ygrid)
			if larger[y][x] > 9 {
				larger[y][x] -= 9
			}
		}
	}

	return dijkstra(larger, position{0, 0}, position{size - 1, size - 1}, 5*size)
}

func main() {
	// Read the input
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	size := len(scanner.Text())
	file.Close()

	file, err = os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	scanner = bufio.NewScanner(file)
	defer file.Close()

	// Create the grid
	grid := make([][]byte, size)
	for i := 0; i < size; i++ {
		grid[i] = make([]byte, size)
	}

	// Fill the grid with the input
	x, y := 0, 0
	for scanner.Scan() {
		line := scanner.Text()
		for _, c := range line {
			grid[y][x] = byte(c) - '0'
			x++
		}
		x = 0
		y++
	}

	fmt.Println(part1(grid, size))
	fmt.Println(part2(grid, size))
}

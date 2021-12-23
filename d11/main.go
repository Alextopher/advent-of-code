package main

import (
	"bufio"
	"fmt"
	"os"
)

const size = 10

func max(x, y int) int {
	if x < y {
		return y
	}
	return x
}

func show(grid [size][size]byte) {
	for y := 0; y < size; y++ {
		for x := 0; x < size; x++ {
			fmt.Print(grid[y][x])
		}
		fmt.Print("\n")
	}
}

func inbounds(x, y int) bool {
	return x >= 0 && x <= size-1 && y >= 0 && y <= size-1
}

func part1() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	var grid [size][size]byte

	scanner := bufio.NewScanner(file)
	i := 0
	for scanner.Scan() {
		for j, c := range scanner.Bytes() {
			grid[i][j] = c - '0'
		}

		i++
	}

	show(grid)
	fmt.Println("--")
	flashes := 0

	for i := 0; i < 100; i++ {
		// first pass add 1 to each cell
		for y := 0; y < size; y++ {
			for x := 0; x < size; x++ {
				grid[y][x] = grid[y][x] + 1
			}
		}

		var flashed [size][size]bool
		for y := 0; y < size; y++ {
			for x := 0; x < size; x++ {
				if grid[y][x] > 9 {
					flashed[y][x] = true
					flashes++
					// add to neighbors
					if inbounds(x-1, y-1) && !flashed[y-1][x-1] {
						grid[y-1][x-1]++
					}
					if inbounds(x, y-1) && !flashed[y-1][x] {
						grid[y-1][x]++
					}
					if inbounds(x+1, y-1) && !flashed[y-1][x+1] {
						grid[y-1][x+1]++
					}
					if inbounds(x-1, y) && !flashed[y][x-1] {
						grid[y][x-1]++
					}
					if inbounds(x+1, y) && !flashed[y][x+1] {
						grid[y][x+1]++
					}
					if inbounds(x-1, y+1) && !flashed[y+1][x-1] {
						grid[y+1][x-1]++
					}
					if inbounds(x, y+1) && !flashed[y+1][x] {
						grid[y+1][x]++
					}
					if inbounds(x+1, y+1) && !flashed[y+1][x+1] {
						grid[y+1][x+1]++
					}
					grid[y][x] = 0

					// After a flash we have to backtrack
					x = -1
					y = 0
				}
			}
		}

		show(grid)
		fmt.Println()

		for y := 0; y < size; y++ {
			for x := 0; x < size; x++ {
				if grid[y][x] > 9 {
					grid[y][x] = 0
				}
			}
		}

		show(grid)
		fmt.Println("--", i+1)
	}

	fmt.Println(flashes)
}

func part2() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	var grid [size][size]byte

	scanner := bufio.NewScanner(file)
	i := 0
	for scanner.Scan() {
		for j, c := range scanner.Bytes() {
			grid[i][j] = c - '0'
		}

		i++
	}

	i = 0
	for {
		// first pass add 1 to each cell
		for y := 0; y < size; y++ {
			for x := 0; x < size; x++ {
				grid[y][x] = grid[y][x] + 1
			}
		}

		flashes := 0
		var flashed [size][size]bool
		for y := 0; y < size; y++ {
			for x := 0; x < size; x++ {
				if grid[y][x] > 9 {
					flashed[y][x] = true
					flashes++
					// add to neighbors
					if inbounds(x-1, y-1) && !flashed[y-1][x-1] {
						grid[y-1][x-1]++
					}
					if inbounds(x, y-1) && !flashed[y-1][x] {
						grid[y-1][x]++
					}
					if inbounds(x+1, y-1) && !flashed[y-1][x+1] {
						grid[y-1][x+1]++
					}
					if inbounds(x-1, y) && !flashed[y][x-1] {
						grid[y][x-1]++
					}
					if inbounds(x+1, y) && !flashed[y][x+1] {
						grid[y][x+1]++
					}
					if inbounds(x-1, y+1) && !flashed[y+1][x-1] {
						grid[y+1][x-1]++
					}
					if inbounds(x, y+1) && !flashed[y+1][x] {
						grid[y+1][x]++
					}
					if inbounds(x+1, y+1) && !flashed[y+1][x+1] {
						grid[y+1][x+1]++
					}
					grid[y][x] = 0

					// After a flash we have to backtrack
					x = -1
					y = 0
				}
			}
		}

		for y := 0; y < size; y++ {
			for x := 0; x < size; x++ {
				if grid[y][x] > 9 {
					grid[y][x] = 0
				}
			}
		}

		show(grid)
		fmt.Println(flashes)
		if flashes == size*size {
			fmt.Println(i + 1)
			break
		}
		i++
	}
}

func main() {
	part1()
	part2()
}

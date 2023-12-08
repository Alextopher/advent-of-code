package main

import (
	"bufio"
	"fmt"
	"os"
)

type Image struct {
	width, height          int
	transformX, transformY int // The true origin is at (transformX, transformY)
	outside                bool
	pixels                 [][]bool
}

func (i *Image) String() string {
	var result string
	for _, line := range i.pixels {
		for _, pixel := range line {
			if pixel {
				result += "■"
			} else {
				result += "□"
			}
		}
		result += "\n"
	}

	return result
}

func main() {
	iterations := 50

	// read input file
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Scan()

	// first line is the image enhancement algorithm
	algorithm := make([]bool, 512)
	for i, c := range scanner.Text() {
		algorithm[i] = c == '#'
	}

	// next lines make up the image
	image := &Image{}
	image.pixels = make([][]bool, 0)

	// scan the empty line
	scanner.Scan()

	// first line
	scanner.Scan()
	image.width = len(scanner.Text()) + iterations*4

	for i := 0; i < 2*iterations; i++ {
		image.pixels = append(image.pixels, make([]bool, image.width))
	}

	// add the first line
	pixels := make([]bool, image.width)
	for i, c := range scanner.Text() {
		pixels[i+2*iterations] = c == '#'
	}
	image.pixels = append(image.pixels, pixels)

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}

		pixels := make([]bool, image.width)
		for i, c := range line {
			pixels[i+2*iterations] = c == '#'
		}
		image.pixels = append(image.pixels, pixels)
	}
	for i := 0; i < 2*iterations; i++ {
		image.pixels = append(image.pixels, make([]bool, image.width))
	}
	image.height = len(image.pixels)

	for i := 0; i < iterations; i++ {
		next := &Image{}
		next.width, next.height = image.width, image.height
		next.pixels = make([][]bool, image.height)

		// figure out what happens to the pixels outside the image
		if image.outside {
			next.outside = algorithm[255]
		} else {
			next.outside = algorithm[0]
		}

		for y := 0; y < image.height; y++ {
			next.pixels[y] = make([]bool, image.width)
			for x := 0; x < image.width; x++ {
				// squares from the top left to the bottom right
				squares := [][2]int{
					{x - 1, y - 1},
					{x, y - 1},
					{x + 1, y - 1},
					{x - 1, y},
					{x, y},
					{x + 1, y},
					{x - 1, y + 1},
					{x, y + 1},
					{x + 1, y + 1},
				}

				mask := 0b100000000
				value := 0

				// we're playing some kind of game of life here
				for _, square := range squares {
					if square[0] < 0 || square[1] < 0 || square[0] >= image.width || square[1] >= image.height {
						// equal to the color of outside of the image
						if image.outside {
							value |= mask
						}
					} else {
						if image.pixels[square[1]][square[0]] {
							value |= mask
						}
					}
					mask >>= 1
				}

				if algorithm[value] {
					next.pixels[y][x] = true
				}
			}
		}

		image = next
		// fmt.Println(image)

		if i == 1 || i == 49 {
			// count the number of on pixels
			count := 0
			for _, line := range image.pixels {
				for _, pixel := range line {
					if pixel {
						count++
					}
				}
			}

			fmt.Println(count)
		}
	}
}

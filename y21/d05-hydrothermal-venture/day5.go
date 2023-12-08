package main

import (
	"bufio"
	"fmt"
	"os"
)

func part1() int {
	var vents [1000 * 1000]int

	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var x1, y1, x2, y2 int
		fmt.Sscanf(scanner.Text(), "%d,%d -> %d,%d", &x1, &y1, &x2, &y2)

		if x1 == x2 {
			if y1 < y2 {
				for y := y1; y <= y2; y++ {
					vents[y*1000+x1] += 1
				}
			} else {
				for y := y2; y <= y1; y++ {
					vents[y*1000+x1] += 1
				}
			}
		} else if y1 == y2 {
			if x1 < x2 {
				for x := x1; x <= x2; x++ {
					vents[y1*1000+x] += 1
				}
			} else {
				for x := x2; x <= x1; x++ {
					vents[y1*1000+x] += 1
				}
			}
		}
	}

	count := 0
	for _, v := range vents {
		if v >= 2 {
			count += 1
		}
	}

	return count
}

func part2() int {
	var vents [1000 * 1000]int

	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var x1, y1, x2, y2 int
		fmt.Sscanf(scanner.Text(), "%d,%d -> %d,%d", &x1, &y1, &x2, &y2)

		if x1 == x2 {
			if y1 < y2 {
				for y := y1; y <= y2; y++ {
					vents[y*1000+x1] += 1
				}
			} else {
				for y := y2; y <= y1; y++ {
					vents[y*1000+x1] += 1
				}
			}
		} else if y1 == y2 {
			if x1 < x2 {
				for x := x1; x <= x2; x++ {
					vents[y1*1000+x] += 1
				}
			} else {
				for x := x2; x <= x1; x++ {
					vents[y1*1000+x] += 1
				}
			}
		} else {
			// yeah the other part wasn't pretty this part won't be either!
			diff := x2 - x1
			if diff < 0 {
				diff = -diff
			}

			if x2 > x1 {
				if y2 > y1 {
					for i := 0; i <= diff; i++ {
						vents[(y1+i)*1000+(x1+i)]++
					}
				} else {
					for i := 0; i <= diff; i++ {
						vents[(y1-i)*1000+(x1+i)]++
					}
				}
			} else {
				if y2 > y1 {
					for i := 0; i <= diff; i++ {
						vents[(y1+i)*1000+(x1-i)]++
					}
				} else {
					for i := 0; i <= diff; i++ {
						vents[(y1-i)*1000+(x1-i)]++
					}
				}
			}
		}
	}

	count := 0
	for _, v := range vents {
		if v >= 2 {
			count += 1
		}
	}

	return count
}

func main() {
	fmt.Println(part1())
	fmt.Println(part2())
}

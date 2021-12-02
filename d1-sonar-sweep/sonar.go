package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func part1() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	last := 0
	count := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		i, _ := strconv.Atoi(scanner.Text())
		// fmt.Printf("%d", i)
		if i > last {
			// fmt.Printf(" increase")
			count++
		}

		last = i
		// fmt.Printf("\n")
	}

	fmt.Println(count - 1)
}

func part2() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	count := 0
	scanner.Scan()
	a, _ := strconv.Atoi(scanner.Text())
	scanner.Scan()
	b, _ := strconv.Atoi(scanner.Text())
	scanner.Scan()
	c, _ := strconv.Atoi(scanner.Text())

	oldSum := a + b + c

	for scanner.Scan() {
		a = b
		b = c
		c, _ = strconv.Atoi(scanner.Text())

		// fmt.Println(a, b, c, "|", oldSum, a+b+c)

		if a+b+c > oldSum {
			count++
		}

		oldSum = a + b + c
	}

	fmt.Println(count)
}

func main() {
	part1()
	part2()
}

package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func part1() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	depth := 0
	pos := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), " ")
		n, _ := strconv.Atoi(split[1])

		if split[0] == "forward" {
			pos += n
		} else if split[0] == "down" {
			depth += n
		} else if split[0] == "up" {
			depth -= n
		}
	}

	fmt.Println(depth * pos)
}

func part2() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	depth := 0
	pos := 0
	aim := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), " ")
		n, _ := strconv.Atoi(split[1])

		if split[0] == "forward" {
			pos += n
			depth += n * aim
		} else if split[0] == "down" {
			aim += n
		} else if split[0] == "up" {
			aim -= n
		}
	}

	fmt.Println(depth * pos)
}

func main() {
	part1()
	part2()
}

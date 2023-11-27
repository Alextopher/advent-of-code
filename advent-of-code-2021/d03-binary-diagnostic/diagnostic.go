package main

import (
	"bufio"
	"container/list"
	"fmt"
	"os"
	"strconv"
)

const size int = 12

func part1() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	var counts [size]int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		n, _ := strconv.ParseInt(scanner.Text(), 2, 0)

		for i := 0; i < size; i++ {
			if n&(1<<i) == 0 {
				counts[size-i-1] -= 1
			} else {
				counts[size-i-1] += 1
			}
		}
	}

	var gamma uint64
	for i := 0; i < size; i++ {
		if counts[i] > 0 {
			gamma |= 1 << (size - i - 1)
		}
	}

	var epsilon uint64
	epsilon = ^gamma & ((1 << size) - 1)

	fmt.Println(gamma * epsilon)
}

func run(list *list.List, invert bool) int64 {
	var i int
	for i = size - 1; list.Len() > 1; i-- {
		count := 0

		// find most common bit in position i
		for e := list.Front(); e != nil; e = e.Next() {
			if e.Value.(int64)&(1<<i) != 0 {
				count++
			} else {
				count--
			}
		}

		// the decision flips for co2
		filter := count >= 0 != invert

		// remove filtered bits
		for e := list.Front(); e != nil; {
			if e.Value.(int64)&(1<<i) == 0 == filter {
				e = e.Next()
				if e == nil {
					list.Remove(list.Back())
				} else {
					list.Remove(e.Prev())
				}
			} else {
				e = e.Next()
			}
		}
	}

	return list.Front().Value.(int64)
}

func part2() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	// Load report into 2 linked lists
	scanner := bufio.NewScanner(file)
	o2 := list.New()
	co2 := list.New()
	for scanner.Scan() {
		n, _ := strconv.ParseInt(scanner.Text(), 2, 0)
		o2.PushBack(n)
		co2.PushBack(n)
	}

	fmt.Println(run(o2, false) * run(co2, true))
}

func main() {
	part1()
	part2()
}

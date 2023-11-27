package main

import (
	"bufio"
	"container/list"
	"fmt"
	"os"
)

func part1() {
	fish := list.New()

	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	reader := bufio.NewReader(file)

	for {
		b, err := reader.ReadByte()
		if err != nil {
			break
		}
		_, err = reader.ReadByte()
		fish.PushBack(int(b - '0'))
	}

	for i := 0; i < 81; i++ {
		for f := fish.Front(); f != nil; f = f.Next() {
			if f.Value.(int) == -1 {
				f.Value = 5
				fish.PushBack(8)
			} else {
				f.Value = f.Value.(int) - 1
			}
		}
	}

	fmt.Println(fish.Len())
}

func part2() {
	fish := make(map[int]int)

	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	reader := bufio.NewReader(file)

	for {
		b, err := reader.ReadByte()
		if err != nil {
			break
		}
		_, err = reader.ReadByte()
		fish[int(b-'0')]++
	}

	fmt.Println(fish)
	for i := 0; i < 256; i++ {
		new := fish[0]
		for i := 0; i < 9; i++ {
			fish[i] = fish[i+1]
		}
		fish[6] += new
		fish[8] = new
	}

	sum := 0
	for _, v := range fish {
		sum += v
	}

	fmt.Println(sum)
}

func main() {
	part1()
	part2()
}

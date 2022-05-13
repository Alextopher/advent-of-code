package main

import (
	"bufio"
	"container/list"
	"fmt"
	"os"
	"sort"
)

func part1() {
	// Read the input
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	// The first line is the polymer
	scanner.Scan()
	polymer := list.New()
	for _, c := range scanner.Text() {
		polymer.PushBack(c)
	}

	// Next line is blank
	scanner.Scan()

	// Each subsequent line is a reaction in the form "AB -> C"
	// I think a map is the best way to do this
	reactions := make(map[rune]map[rune]rune)
	for scanner.Scan() {
		var a, b, c rune
		fmt.Sscanf(scanner.Text(), "%c%c -> %c", &a, &b, &c)
		if _, ok := reactions[a]; !ok {
			reactions[a] = make(map[rune]rune)
		}
		if _, ok := reactions[b]; !ok {
			reactions[b] = make(map[rune]rune)
		}
		reactions[a][b] = c
	}

	// run for 10 rounds
	for i := 0; i < 10; i++ {
		// Scan through the polymer
		itr := polymer.Front()
		for itr.Next() != nil {
			a := itr.Value.(rune)
			b := itr.Next().Value.(rune)

			// Look up the reaction
			if _, ok := reactions[a]; ok {
				if _, ok := reactions[a][b]; ok {
					// Add the new element to the polymer
					polymer.InsertAfter(reactions[a][b], itr)
					// move forward two elements
					itr = itr.Next().Next()
					continue
				}
			}

			fmt.Println("No reaction for", string(a), string(b))
			os.Exit(1)
		}

		// PrintRuneListAsString(polymer)
	}

	// count the number of elements in the polymer
	counts := make(map[rune]int)
	for itr := polymer.Front(); itr != nil; itr = itr.Next() {
		counts[itr.Value.(rune)]++
	}

	// convert to a sorted list of integers
	var sorted []int
	for _, v := range counts {
		sorted = append(sorted, v)
	}
	sort.Ints(sorted)

	// take the difference between the first and last element
	fmt.Println(sorted[len(sorted)-1] - sorted[0])

	s := RuneListAsString(polymer)
	pairs := make(map[pair]int)
	for i := 0; i < len(s)-1; i++ {
		c1 := s[i]
		c2 := s[i+1]
		pairs[pair{c1, c2}]++
	}
}

func RuneListAsString(l *list.List) string {
	var s string
	for itr := l.Front(); itr != nil; itr = itr.Next() {
		s += string(itr.Value.(rune))
	}
	return s
}

type pair struct {
	a, b byte
}

func part2() {
	// Read the input
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	// The first line is the polymer
	scanner.Scan()

	lastElement := scanner.Text()[len(scanner.Text())-1]
	pairs := make(map[pair]int)
	for i := 0; i < len(scanner.Text())-1; i++ {
		c1 := scanner.Text()[i]
		c2 := scanner.Text()[i+1]
		pairs[pair{c1, c2}]++
	}

	// Next line is blank
	scanner.Scan()

	// Each subsequent line is a reaction in the form "AB -> C"
	// I think a map is the best way to do this
	reactions := make(map[pair]byte)

	for scanner.Scan() {
		var a, b, c byte
		_, err := fmt.Sscanf(scanner.Text(), "%c%c -> %c", &a, &b, &c)
		if err != nil {
			fmt.Println(err)
			break
		}
		reactions[pair{a, b}] = c
	}

	nextPairs := make(map[pair]int)
	for i := 0; i < 40; i++ {
		// scan through the current pairs
		for p, v := range pairs {
			if v == 0 {
				continue
			}
			// look up the reaction
			if r, ok := reactions[p]; ok {
				// add the new pair to the next pairs
				nextPairs[pair{p.a, r}] += v
				nextPairs[pair{r, p.b}] += v
			} else {
				fmt.Println("No reaction for", string(p.a), string(p.b))
				os.Exit(1)
			}
		}

		// swap the current and next pairs
		pairs = nextPairs
		nextPairs = make(map[pair]int)
	}

	// count the number of elements in the polymer
	counts := make(map[byte]int)
	for p, v := range pairs {
		// To avoid counting the same pair twice, we only count the first
		counts[p.a] += v
	}
	// we need to manually add the last element
	counts[lastElement]++

	// convert to a sorted list of integers
	var sorted []int
	for _, v := range counts {
		sorted = append(sorted, v)
	}
	sort.Ints(sorted)

	// take the difference between the first and last element
	fmt.Println(sorted[len(sorted)-1] - sorted[0])
}

func main() {
	part1()
	part2()
}

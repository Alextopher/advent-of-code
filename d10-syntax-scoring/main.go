package main

import (
	"bufio"
	"container/list"
	"fmt"
	"os"
	"sort"
)

// {([(<[}>{{[(
func part1() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	var illegal []string

	scanner := bufio.NewScanner(file)
	score := 0
	for scanner.Scan() {
		stack := list.New()

		for _, char := range scanner.Text() {
			if char == '(' || char == '[' || char == '{' || char == '<' {
				stack.PushBack(char)
			} else {
				back := stack.Back().Value.(rune)

				if char == ')' {
					if back == '(' {
						stack.Remove(stack.Back())
					} else {
						illegal = append(illegal, scanner.Text())
						score += 3
						break
					}
				} else if char == ']' {
					if back == '[' {
						stack.Remove(stack.Back())
					} else {
						illegal = append(illegal, scanner.Text())
						score += 57
						break
					}
				} else if char == '}' {
					if back == '{' {
						stack.Remove(stack.Back())
					} else {
						illegal = append(illegal, scanner.Text())
						score += 1197
						break
					}
				} else if char == '>' {
					if back == '<' {
						stack.Remove(stack.Back())
					} else {
						illegal = append(illegal, scanner.Text())
						score += 25137
						break
					}
				}
			}
		}
	}

	fmt.Println(score)
}

func part2() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var scores []int
	for scanner.Scan() {
		stack := list.New()

		illegal := false
		for _, char := range scanner.Text() {
			if char == '(' || char == '[' || char == '{' || char == '<' {
				stack.PushBack(char)
			} else {
				back := stack.Back().Value.(rune)

				if char == ')' {
					if back == '(' {
						stack.Remove(stack.Back())
					} else {
						illegal = true
						break
					}
				} else if char == ']' {
					if back == '[' {
						stack.Remove(stack.Back())
					} else {
						illegal = true
						break
					}
				} else if char == '}' {
					if back == '{' {
						stack.Remove(stack.Back())
					} else {
						illegal = true
						break
					}
				} else if char == '>' {
					if back == '<' {
						stack.Remove(stack.Back())
					} else {
						illegal = true
						break
					}
				}
			}
		}

		if !illegal {
			// empty the stack
			score := 0
			for stack.Len() != 0 {
				back := stack.Back().Value.(rune)
				score *= 5
				switch back {
				case '(':
					score += 1
					break
				case '[':
					score += 2
					break
				case '{':
					score += 3
					break
				case '<':
					score += 4
					break
				default:
					panic("invalid character")
				}

				stack.Remove(stack.Back())
			}

			scores = append(scores, score)
		}
	}

	// score scores
	sort.Ints(scores)
	fmt.Println(scores[len(scores)/2])
}

func main() {
	part1()
	part2()
}

package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func part1() int {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	var reactor [101][101][101]bool
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var isOn bool
		var x1, x2, y1, y2, z1, z2 int

		split := strings.Split(scanner.Text(), " ")

		if split[0] == "on" {
			isOn = true
		} else if split[0] == "off" {
			isOn = false
		} else {
			fmt.Println("Invalid on/off", split[0])
			os.Exit(1)
		}

		fmt.Sscanf(split[1], "x=%d..%d,y=%d..%d,z=%d..%d", &x1, &x2, &y1, &y2, &z1, &z2)

		for x := x1; x <= x2; x++ {
			if x < -50 || x > 50 {
				break
			}
			for y := y1; y <= y2; y++ {
				if y < -50 || y > 50 {
					break
				}
				for z := z1; z <= z2; z++ {
					if z < -50 || z > 50 {
						break
					}
					reactor[x+50][y+50][z+50] = isOn
				}
			}
		}
	}

	count := 0
	for x := -50; x <= 50; x++ {
		for y := -50; y <= 50; y++ {
			for z := -50; z <= 50; z++ {
				if reactor[x+50][y+50][z+50] {
					count++
				}
			}
		}
	}

	return count
}

func min(a, b int) int {
	if a < b {
		return a
	} else {
		return b
	}
}

func max(a, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}

type Interval struct {
	Start int
	End   int
}

func (i *Interval) size() int {
	return i.End - i.Start + 1
}

func intervalIntersects(i, j *Interval) bool {
	return i.Start <= j.End && i.End >= j.Start
}

// ------|-------|----
// ---|-------|-------
// ------|----| ------
func intervalIntersection(i, j *Interval) *Interval {
	return &Interval{
		Start: max(i.Start, j.Start),
		End:   min(i.End, j.End),
	}
}

// Retangular Prism
type Prism struct {
	isOn    bool
	x, y, z Interval
}

func (r *Prism) size() int {
	volume := r.x.size() * r.y.size() * r.z.size()

	if r.isOn {
		return volume
	} else {
		return -volume
	}
}

func prismIntersects(r1, r2 *Prism) bool {
	return intervalIntersects(&r1.x, &r2.x) && intervalIntersects(&r1.y, &r2.y) && intervalIntersects(&r1.z, &r2.z)
}

func prismIntersection(r1, r2 *Prism) *Prism {
	return &Prism{!r1.isOn, *intervalIntersection(&r1.x, &r2.x), *intervalIntersection(&r1.y, &r2.y), *intervalIntersection(&r1.z, &r2.z)}
}

func part2() int {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	var prisms []Prism
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var isOn bool
		var x1, x2, y1, y2, z1, z2 int

		split := strings.Split(scanner.Text(), " ")

		if split[0] == "on" {
			isOn = true
		} else if split[0] == "off" {
			isOn = false
		} else {
			fmt.Println("Invalid on/off", split[0])
			os.Exit(1)
		}

		fmt.Sscanf(split[1], "x=%d..%d,y=%d..%d,z=%d..%d", &x1, &x2, &y1, &y2, &z1, &z2)
		prisms = append(prisms, Prism{isOn, Interval{x1, x2}, Interval{y1, y2}, Interval{z1, z2}})
	}

	// This is the "find all of the intersections part"
	// Game engines have figured out some nice O(n log n) stuff for this like quad trees or something
	// Here is O(n^2)
	var volumes []Prism

	for _, prism := range prisms {
		// Add intersections with all existing volumes
		var results []Prism

		for _, volume := range volumes {
			if prismIntersects(&volume, &prism) {
				results = append(results, *prismIntersection(&volume, &prism))
			}
		}

		volumes = append(volumes, results...)

		// Add prism if it's on
		if prism.isOn {
			volumes = append(volumes, prism)
		}
	}

	sum := 0
	for _, volume := range volumes {
		sum += volume.size()
	}

	return sum
}

func main() {
	fmt.Println(part1(), part2())
}

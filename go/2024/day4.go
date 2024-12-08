package twentyfour

import (
	"strconv"
	"strings"
)

func init() {
	AOC24Solutions.RegisterSolution(0, 4, Day4Part1)
	AOC24Solutions.RegisterSolution(1, 4, Day4Part2)
}

func parseDay4Input(input string) ([]string, error) {
	a := strings.Split(input, "\n")
	a = a[:len(a)-1]
	return a, nil
}

func getOrDefault(grid []string, x, y int) byte {
	if x < 0 || x >= len(grid[0]) || y < 0 || y >= len(grid) {
		return 0
	}
	return grid[y][x]
}

type Pair struct {
	X int
	Y int
}

func findXmas(grid []string, x, y int) int {
	if grid[y][x] != 'X' {
		return 0
	}

	// find M
	coordinates := []Pair{}
	miny := max(0, y-1)
	maxy := min(len(grid)-1, y+1)
	minx := max(0, x-1)
	maxx := min(len(grid[0])-1, x+1)
	for yy := miny; yy <= maxy; yy++ {
		for xx := minx; xx <= maxx; xx++ {
			if grid[yy][xx] == 'M' {
				coordinates = append(coordinates, Pair{xx, yy})
			}
		}
	}

	// find remainder
	letters := []byte("AS")
	count := 0
	for _, coord := range coordinates {
		dx, dy := coord.X-x, coord.Y-y
		isValid := true
		for i, letter := range letters {
			o := i + 2
			if getOrDefault(grid, x+dx*o, y+dy*o) != letter {
				isValid = false
				break
			}
		}

		if isValid {
			count++
		}
	}

	return count
}

// Day4Part1 is the entry point for Day 4 part 1 solutions.
func Day4Part1(input string) (string, error) {
	grid, err := parseDay4Input(input)
	if err != nil {
		return "", err
	}

	count := 0
	for y, line := range grid {
		for x := range line {
			// call findXmas
			count += findXmas(grid, x, y)
		}
	}

	return strconv.Itoa(count), nil
}

func part2Check(grid []string, x, y int) bool {
	// get first diagonal
	for dx := -1; dx <= 1; dx += 2 {
		for dy := -1; dy <= 1; dy += 2 {
			if getOrDefault(grid, x+dx, y+dy) != 'M' && getOrDefault(grid, x+dx, y+dy) != 'S' {
				return false
			}
		}
	}

	if getOrDefault(grid, x-1, y-1)+getOrDefault(grid, x+1, y+1) != 'M'+'S' {
		return false
	}

	if getOrDefault(grid, x+1, y-1)+getOrDefault(grid, x-1, y+1) != 'M'+'S' {
		return false
	}

	return true
}

// Day4Part2 is the entry point for Day 4 part 2 solutions.
func Day4Part2(input string) (string, error) {
	grid, err := parseDay4Input(input)
	if err != nil {
		return "", err
	}

	// go through the grid to find an 'A'
	count := 0
	for y, line := range grid {
		for x := range line {
			if grid[y][x] == 'A' {
				// is it an XMAS
				if part2Check(grid, x, y) {
					count++
				}
			}
		}
	}

	return strconv.Itoa(count), nil
}

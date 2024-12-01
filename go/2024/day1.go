package twentyfour

import (
	"sort"
	"strconv"
	"strings"

)


func init() {
	AOC24Solutions.RegisterSolution(0, 1, Day1Part1)
	AOC24Solutions.RegisterSolution(1, 1, Day1Part2)
}

func distance(a, b int) int {
	if a < b {
		return b - a
	}
	return a - b
}

func parseDay1(input string) ([]int, []int, error) {
	list1 := make([]int, 0)
	list2 := make([]int, 0)

	for i, field := range strings.Fields(input) {
		fieldInt, err := strconv.Atoi(field)
		if err != nil {
			return nil, nil, err
		}

		if i % 2 == 0 {
			list1 = append(list1, fieldInt)
		} else {
			list2 = append(list2, fieldInt)
		}
	}

	return list1, list2, nil
}

// Day1Part1 is the entry point for Day 1 part 1 solutions.
func Day1Part1(input string) (string, error) {
	list1, list2, err := parseDay1(input)
	if err != nil {
		return "", err
	}

	// Sort each list.
	sort.Ints(list1)
	sort.Ints(list2)

	// Get the difference between list 1 and list 2.
	soln := 0
	for i := range list1 {
		soln += distance(list1[i], list2[i])
	}
	
	return strconv.Itoa(soln), nil
}

// Day1Part2 is the entry point for Day 1 part 2 solutions.
func Day1Part2(input string) (string, error) {
	list1, list2, err := parseDay1(input)
	if err != nil {
		return "", err
	}
	
	counts := make(map[int]int)
	for _, elem := range list2 {
		counts[elem]++
	}
	
	similarity := 0
	for _, elem := range list1 {
		similarity += counts[elem] * elem
	}
	
	return strconv.Itoa(similarity), nil
}

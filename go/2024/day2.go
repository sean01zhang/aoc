package twentyfour

import (
	"fmt"
	"strconv"
	"strings"
)

func init() {
	AOC24Solutions.RegisterSolution(0, 2, Day2Part1)
	AOC24Solutions.RegisterSolution(1, 2, Day2Part2)
}

func parseDay2Input(input string) ([][]int, error) {
	output := make([][]int, 0)
	input = strings.TrimSpace(input)

	for i, line := range strings.Split(input, "\n") {
		output = append(output, make([]int, 0))
		for _, field := range strings.Fields(line) {
			fieldInt, err := strconv.Atoi(field)
			if err != nil {
				return nil, err
			}

			output[i] = append(output[i], fieldInt)
		}
	}

	return output, nil
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func inRange(a, l, r int) bool {
	return a >= l && a < r
}

func isSafe(report []int) bool {
	inc := report[0] < report[1]

	prev := report[0]
	for _, val := range report[1:] {
		if (inc && val <= prev) || (!inc && val >= prev) || !inRange(abs(val-prev), 1, 4) {
			return false
		}
		prev = val
	}

	return true
}

func Day2Part1(input string) (string, error) {
	reports, err := parseDay2Input(input)
	if err != nil {
		return "", err
	}

	countSafe := 0
	for _, report := range reports {
		if isSafe(report) {
			countSafe++
		}
	}

	return fmt.Sprintf("%d", countSafe), nil
}

func isIncreasing(report []int, faults int) bool {
	prev := report[0]
	for _, val := range report[1:] {
		if val < prev {
			faults--
		}
		prev = val
	}

	return faults >= 0
}

func validate(prev, cur int, inc bool) bool {
	return !((inc && cur <= prev) || (!inc && cur >= prev) || !inRange(abs(cur-prev), 1, 4))
}

func isSafeWithFault(report []int) bool {
	inc := report[0] < report[1]

	faults := 0
	prev := report[0]
	for i, val := range report[1:] {
		i := i + 1
		if !validate(prev, val, inc) {
			faults += 1

			// remove val
			if i+1 >= len(report) || validate(prev, report[i+1], inc) {
				// chosen to remove
				if i+1 < len(report) {
					inc = prev < report[i+1]
				} else {
					inc = report[i-2] < prev
				}

				continue
			// remove prev
			} else if i-2 < 0 || validate(report[i-2], val, inc) {
				// chosen to remove
				if i-2 >= 0 {
					inc = report[i-2] < val
				} else {
					inc = val < report[i+1]
				}

			// don't do anything
			} else {
				return false
			}
		}
		prev = val
	}

	return faults <= 1
}

func Day2Part2(input string) (string, error) {
	reports, err := parseDay2Input(input)
	if err != nil {
		return "", err
	}

	countSafe := 0
	for _, report := range reports {
		if isSafeWithFault(report) {
			countSafe++
		}
	}

	return fmt.Sprintf("%d", countSafe), nil
}

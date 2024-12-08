package twentyfour

import (
	"regexp"
	"strconv"
)

func init() {
	AOC24Solutions.RegisterSolution(0, 3, Day3Part1)
	AOC24Solutions.RegisterSolution(1, 3, Day3Part2)
}

func mul(input string) (int, error) {
	re := regexp.MustCompile(`\d{1,3}`)
	nums := re.FindAllString(input, 2)

	num1, err := strconv.Atoi(nums[0])
	if err != nil {
		return 0, err
	}

	num2, err := strconv.Atoi(nums[1])
	if err != nil {
		return 0, err
	}

	return num1 * num2, nil
}

func mustMul(input string) int {
	result, err := mul(input)
	if err != nil {
		panic(err)
	}
	return result
}

func Day3Part1(input string) (string, error) {
	re := regexp.MustCompile(`mul\(\d{1,3},\d{1,3}\)`)
	
	sum := 0
	matches := re.FindAllString(input, -1)
	for _, match := range matches {
		sum += mustMul(match)
	}

	return strconv.Itoa(sum), nil
}

func Day3Part2(input string) (string, error) {
	re := regexp.MustCompile(`(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))`)
	
	sum := 0
	doAdd := true
	for _, match := range re.FindAllString(input, -1) {
		// If the match is a mul, add it to the sum.
		if match[0] == 'm' {
			if doAdd {
				sum += mustMul(match)
			}
		} else if match[2] == 'n' {
			doAdd = false
		} else {
			doAdd = true
		}
	}

	return strconv.Itoa(sum), nil
}

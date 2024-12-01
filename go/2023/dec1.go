package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("./input/dec1.txt")
	defer file.Close()
	if err != nil {
		fmt.Println(err)
	}

	numbers := [10]string{"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

	scanner := bufio.NewScanner(file)
	total := 0
	for scanner.Scan() {
		text := scanner.Text()
		nums := make([]int, 0)
		for i, char := range text {
			if char >= '0' && char <= '9' {
				nums = append(nums, int(char-'0'))
			} else {
				for val, number := range numbers {
					if strings.HasPrefix(text[i:], number) {
						nums = append(nums, val)
					}
				}
			}
		}
		total += nums[0]*10 + nums[len(nums)-1]
	}

	fmt.Println(total)
}

package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	p2()
}

func p1() {
	file, err := os.Open("./input/dec4.txt")
	defer file.Close()
	if err != nil {
		fmt.Println("bruh", err)
	}

	scanner := bufio.NewScanner(file)

	answer := 0
	for i := 0; scanner.Scan(); i++ {
		line := scanner.Text()
		content := strings.Split(line[7+len(fmt.Sprintf("%d", i)):], " | ")
		winning := make(map[string]bool)

		points := 0

		for _, win := range strings.Fields(content[0]) {
			winning[win] = true
		}

		for _, yours := range strings.Fields(content[1]) {
			if winning[yours] {
				if points == 0 {
					points = 1
				} else {
					points *= 2
				}
			}
		}

		answer += points
		i += 1
	}

	fmt.Println("D4P1", answer)
}

func p2() {
	file, err := os.Open("./input/dec4.txt")
	defer file.Close()
	if err != nil {
		fmt.Println("bruh", err)
	}

	scanner := bufio.NewScanner(file)

	instances := make([]int, 0)
	for i := 0; scanner.Scan(); i++ {
		if len(instances) == i {
			instances = append(instances, 1)
		} else {
			instances[i] += 1
		}
		line := scanner.Text()
		content := strings.Split(line[7+len(fmt.Sprintf("%d", i)):], " | ")
		winning := make(map[string]bool)

		matches := 0

		for _, win := range strings.Fields(content[0]) {
			winning[win] = true
		}

		for _, yours := range strings.Fields(content[1]) {
			if winning[yours] {
				matches += 1
				if len(instances) == i+matches {
					instances = append(instances, instances[i])
				} else {
					instances[i+matches] += instances[i]
				}
			}
		}
	}

	answer := 0
	for _, count := range instances {
		answer += count
	}

	fmt.Println("D4P2", answer)
}

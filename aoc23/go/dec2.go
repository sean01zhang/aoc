package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("hello world")
	p1()
	p2()
}

func p1() {
	file, err := os.Open("./input/dec2.txt")
	defer file.Close()
	if err != nil {
		fmt.Println("bruh", err)
	}

	// limits
	limits := map[string]int{
		"red":   12,
		"green": 13,
		"blue":  14,
	}

	scanner := bufio.NewScanner(file)
	gameId := 1

	solution := 0

	// for each line
	for scanner.Scan() {
		line := scanner.Text()

		isGameIllegal := false

		splitRegex := regexp.MustCompile("(: )|(; )|(, )")
		counts := splitRegex.Split(line, -1)
		for _, count := range counts[1:] {
			numberAndColor := strings.Split(count, " ")
			number, err := strconv.Atoi(numberAndColor[0])
			if err != nil {
				fmt.Println("Bruh not number")
				panic(err)
			}
			color := numberAndColor[1]
			if number > limits[color] {
				isGameIllegal = true
				break
			}
		}

		// determine red, green, blue
		if !isGameIllegal {
			solution += gameId
		}

		// update game id
		gameId += 1
	}

	fmt.Println("D2P1:", solution)
}

func p2() {
	file, err := os.Open("./input/dec2.txt")
	defer file.Close()
	if err != nil {
		fmt.Println("bruh", err)
	}

	scanner := bufio.NewScanner(file)

	solution := 0

	// for each line
	for scanner.Scan() {
		max := map[string]int{
			"red":   0,
			"green": 0,
			"blue":  0,
		}
		line := scanner.Text()

		splitRegex := regexp.MustCompile("(: )|(; )|(, )")
		counts := splitRegex.Split(line, -1)
		for _, count := range counts[1:] {
			numberAndColor := strings.Split(count, " ")
			number, err := strconv.Atoi(numberAndColor[0])
			if err != nil {
				fmt.Println("Bruh not number")
				panic(err)
			}
			color := numberAndColor[1]

			if max[color] < number {
				max[color] = number
			}
		}

		solution += max["red"] * max["green"] * max["blue"]
	}

	fmt.Println("D2P2:", solution)
}

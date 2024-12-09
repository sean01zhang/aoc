package twentyfour

import (
	"bufio"
	"io"
	"slices"
	"strconv"
	"strings"

	"src.naesna.es/aoc/go/internal/util"
)

func init() {
	AOC24Solutions.RegisterSolution(0, 5, Day5Part1)
	AOC24Solutions.RegisterSolution(1, 5, Day5Part2)
}

func parseDay5Input(input string) (map[int][]int, [][]int, error) {
	r := bufio.NewReader(strings.NewReader(input))

	// befores
	befores := make(map[int][]int)
	for {
		b, err := r.ReadBytes('\n')
		if err != nil {
			return nil, nil, err
		}
		if b[0] == '\n' {
			break
		}

		lStr, rStr := string(b[:2]), string(b[3:5])
		l, err := strconv.Atoi(lStr)
		if err != nil {
			return nil, nil, err
		}

		r, err := strconv.Atoi(rStr)
		if err != nil {
			return nil, nil, err
		}

		befores[r] = append(befores[r], l)
	}

	// getting the instructions
	instructions := make([][]int, 0)
	for {
		b, err := r.ReadBytes('\n')
		if err == io.EOF {
			break
		} else if err != nil {
			return nil, nil, err
		}

		instructions = append(instructions, make([]int, 0))
		for _, n := range strings.Split(strings.TrimSpace(string(b)), ",") {
			i := len(instructions) - 1
			nInt, err := strconv.Atoi(n)
			if err != nil {
				return nil, nil, err
			}

			instructions[i] = append(instructions[i], nInt)
		}
	}

	return befores, instructions, nil
}

// Day5Part1 is the entry point for Day 5 part 1 solutions.
func Day5Part1(input string) (string, error) {
	befores, instructions, err := parseDay5Input(input)
	if err != nil {
		return "", err
	}

	sumMiddles := 0
	for _, instr := range instructions {
		pageMap := make(map[int]int)
		isValid := true

		for i := len(instr) - 1; i >= 0; i-- {
			page := instr[i]
			pageMap[page] = i

			for _, before := range befores[page] {
				if j, ok := pageMap[before]; ok {
					if i < j {
						isValid = false
						break
					}
				}
			}

			if !isValid {
				break
			}
		}

		if isValid {
			sumMiddles += instr[len(instr)/2]
		}

	}

	return strconv.Itoa(sumMiddles), nil
}

// Day5Part2 is the entry point for Day 5 part 2 solutions.
func Day5Part2(input string) (string, error) {
	befores, instructions, err := parseDay5Input(input)
	if err != nil {
		return "", err
	}

	sumMedians := 0
	for _, instruction := range instructions {
		// Create a graph from pages in instruction.
		graph := util.NewDAGraph()
		for _, page := range instruction {
			for _, before := range befores[page] {
				graph.AddEdge(before, page)
			}
		}

		// Get the topology.
		topology := graph.TopologicalSort()
		pageToTopoIndex := make(map[int]int)
		for i, page := range topology {
			pageToTopoIndex[page] = i
		}

		instrTopoIdxs := make([]int, len(instruction))
		for i, page := range instruction {
			instrTopoIdxs[i] = pageToTopoIndex[page]
		}

		// Check if sorted already.
		if !slices.IsSorted(instrTopoIdxs) {
			slices.Sort(instrTopoIdxs)
			sumMedians += topology[instrTopoIdxs[len(instrTopoIdxs)/2]]
		}
	}

	return strconv.Itoa(sumMedians), nil
}

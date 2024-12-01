package twentyfour

import (
	"src.naesna.es/aoc/go/internal/solution"
)

var AOC24Solutions AOC24Client

func init() {
	AOC24Solutions = AOC24Client{}

	for part := range AOC24Solutions.Solutions {
		for day := range AOC24Solutions.Solutions[part] {
			AOC24Solutions.Solutions[part][day] = solution.NotImplemented
		}
	}
}

type Solution=solution.Solution

type AOC24Client struct {
	Solutions [2][24]Solution
}

func (c *AOC24Client) GetSolution(part int, day int) Solution {
	return c.Solutions[part][day - 1]
}

func (c *AOC24Client) RegisterSolution(part int, day int, s Solution) {
	c.Solutions[part][day - 1] = s
}

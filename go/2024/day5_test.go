package twentyfour_test

import (
	"testing"

	twentyfour "src.naesna.es/aoc/go/2024"
)

func TestDay5Part1(t *testing.T) {
	input := SetupInput("ex5")

	got, err := twentyfour.Day5Part1(input)
	if err != nil {
		t.Errorf("Day5Part1() error = %v; want nil", err)
	}
	if want := "143"; got != want {
		t.Errorf("Day5Part1() = %s; want %s", got, want)
	}
}

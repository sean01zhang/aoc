package twentyfour_test

import (
	twentyfour "src.naesna.es/aoc/go/2024"
	"testing"
)

func TestDay3Part1(t *testing.T) {
	input := SetupInput("ex3")

	got, err := twentyfour.Day3Part1(input)
	if err != nil {
		t.Errorf("Day3Part1() error = %v; want nil", err)
	}
	if want := "161"; got != want {
		t.Errorf("Day3Part1() = %s; want %s", got, want)
	}
}

func TestDay3Part2(t *testing.T) {
	input := SetupInput("ex3-2")

	got, err := twentyfour.Day3Part2(input)
	if err != nil {
		t.Errorf("Day3Part2() error = %v; want nil", err)
	}
	if want := "48"; got != want {
		t.Errorf("Day3Part2() = %s; want %s", got, want)
	}
}

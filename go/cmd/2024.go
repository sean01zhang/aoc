package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
	twentyfour "src.naesna.es/aoc/go/2024"
	"src.naesna.es/aoc/go/internal/util"
)

const urlFmt = "https://adventofcode.com/%d/day/%d/input"

var (
	inputFile    string
	sessionToken string

	day int
)

var twentyFourCmd = &cobra.Command{
	Use:   "2024",
	Short: "AoC 2024 Solutions",
	Long:  `Advent of Code 2024 Solutions`,
	Run: func(cmd *cobra.Command, args []string) {
		input := ""
		var err error
		if inputFile != "" {
			input, err = util.GetInputFromFile(inputFile)
		} else {
			input, err = util.GetInputFromOnline(fmt.Sprintf(urlFmt, 2024, day), util.WithSession(sessionToken))
		}
		if err != nil {
			fmt.Println(err)
			return
		}

		for part := 0; part < 2; part++ {
			fmt.Printf("Part %d:\n", part+1)
			sol, err := twentyfour.AOC24Solutions.GetSolution(part, day)(input)
			if err != nil {
				fmt.Println(err)
				return
			}
			fmt.Println(sol)
		}
	},
}

func init() {
	rootCmd.AddCommand(twentyFourCmd)

	// Install flags
	twentyFourCmd.Flags().StringVarP(&inputFile, "input", "i", "", "Input file to run solution on")
	twentyFourCmd.MarkFlagFilename("input")
	twentyFourCmd.Flags().StringVarP(&sessionToken, "session", "s", "", "Session cookie for fetching input from URL")
	twentyFourCmd.Flags().IntVarP(&day, "day", "d", 1, "Day to run solution for")

	twentyFourCmd.MarkFlagRequired("day")
	twentyFourCmd.MarkFlagsOneRequired("input", "session")
	twentyFourCmd.MarkFlagsMutuallyExclusive("input", "session")
}

package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "aoc",
	Short: "Advent of Code CLI",
	Long:  `Advent of Code CLI`,
	Args:  cobra.MinimumNArgs(1),
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

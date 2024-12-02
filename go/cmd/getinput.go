package cmd

import (
	"fmt"
	"io"
	"os"

	"github.com/spf13/cobra"
	"src.naesna.es/aoc/go/internal/util"
)

const urlFmt = "https://adventofcode.com/%s/day/%s/input"

var (
	getInputSessionToken string
	saveInputFilePath    string
)

var getInputCmd = &cobra.Command{
	Use:   "get [year] [day]",
	Short: "Get input for a given day and year",
	Args:  cobra.ExactArgs(2),
	Run: func(cmd *cobra.Command, args []string) {

		year := args[0]
		day := args[1]
		input, err := util.GetInputFromOnline(fmt.Sprintf(urlFmt, year, day), util.WithSession(getInputSessionToken))
		if err != nil {
			fmt.Println(err)
			return
		}

		// Write input to a file
		if saveInputFilePath != "" {
			f, err := os.Open(saveInputFilePath)
			if err != nil {
				fmt.Println(err)
				return
			}

			_, err = io.WriteString(f, input)
			if err != nil {
				fmt.Println(err)
				return
			}
		} else {
			fmt.Println(input)
		}
	},
}

func init() {
	rootCmd.AddCommand(getInputCmd)

	getInputCmd.Flags().StringVarP(&getInputSessionToken, "session", "s", "", "Session cookie for fetching input from URL")
	getInputCmd.MarkFlagRequired("session")
	getInputCmd.Flags().StringVarP(&saveInputFilePath, "file", "f", "", "File to save input to")
}

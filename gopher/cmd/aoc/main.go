package main

import (
	"os"
	"strconv"

	"github.com/iiharsha/aoc/gopher/internal/aoclib"
	"github.com/iiharsha/aoc/gopher/internal/day1"
	"github.com/iiharsha/aoc/gopher/internal/day2"
)

func main() {
	args := os.Args[1:]
	var filteredArgs []string
	aoclib.CurrentInputType = aoclib.MainInput

	for _, arg := range args {
		if arg == "--test" || arg == "-t" {
			aoclib.CurrentInputType = aoclib.TestInput
			continue
		}
		filteredArgs = append(filteredArgs, arg)
	}
	args = filteredArgs

	var selected aoclib.Selector
	switch {
	case len(args) == 0:
		selected = aoclib.Selector{Kind: aoclib.SelectLast}

	case args[0] == "-a" || args[0] == "--all":
		selected = aoclib.Selector{Kind: aoclib.SelectAll}

	default:
		day, err := strconv.Atoi(args[0])
		if err != nil {
			return
		}
		selected = aoclib.Selector{Kind: aoclib.SelectOne, Day: day}
	}

	run2025(selected)
}

func run2025(selected aoclib.Selector) {
	days := []aoclib.Runner{
		&day1.Day1{},
		&day2.Day2{},
	}

	switch selected.Kind {
	case aoclib.SelectLast:
		aoclib.RunSolution(days[len(days)-1])
	case aoclib.SelectAll:
		for _, d := range days {
			aoclib.RunSolution(d)
		}
	case aoclib.SelectOne:
		aoclib.RunSolution(days[selected.Day-1])
	}
}

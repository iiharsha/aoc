package aoclib

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"time"
)

type Runner interface {
	Name() (year int, day int)
	Parse()
	PartOne() []string
	PartTwo() []string
}

type SelectorKind int

const (
	SelectAll SelectorKind = iota
	SelectOne
	SelectLast
)

type Selector struct {
	Kind SelectorKind
	Day  int
}

func ReadLines(pathName string) []string {
	file, err := os.Open(pathName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var lines []string

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		if line != "" {
			lines = append(lines, line)
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return lines
}

func Output[T any](v T) []string {
	return []string{fmt.Sprint(v)}
}

func RunSolution(r Runner) {
	year, day := r.Name()
	fmt.Printf("--- Advent of Code Golang %d, %d ---\n", year, day)

	start := time.Now()
	r.Parse()
	printTiming(time.Since(start))

	start = time.Now()
	partOne := r.PartOne()
	printSolution(1, partOne, time.Since(start))

	start = time.Now()
	partTwo := r.PartTwo()
	printSolution(2, partTwo, time.Since(start))
}

func printTiming(d time.Duration) {
	ms := d.Milliseconds()
	fmt.Printf("Parsed in %3d.%03d \n", ms/1000, ms%1000)
}

func printSolution(part int, solution []string, d time.Duration) {
	ms := d.Milliseconds()

	fmt.Printf("%3d.%03dsecs Part %d: %s\n", ms/1000, ms%1000, part, solution[0])
}

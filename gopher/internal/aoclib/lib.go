package aoclib

import (
	"bufio"
	"embed"
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)

//go:embed input/**
var inputFile embed.FS

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

type InputType int

const (
	MainInput InputType = iota
	TestInput
)

var CurrentInputType InputType = MainInput

func ReadDayLines(year, day int) ([]string, error) {
	if year < 2000 {
		return nil, errors.New("invalid year provided")
	}
	path := fmt.Sprintf("input/%d/day_%d.txt", year, day)
	if CurrentInputType == TestInput {
		path = fmt.Sprintf("input/%d/tests/day_%d.txt", year, day)
	}

	fmt.Println(path)

	data, err := inputFile.ReadFile(path)
	if err != nil {
		panic("something went wrong while embedding input file")
	}

	return strings.Split(strings.TrimSpace(string(data)), "\n"), nil
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

func MustUint(s string) uint {
	v, err := strconv.ParseUint(s, 10, 32)
	if err != nil {
		panic(err)
	}

	return uint(v)
}

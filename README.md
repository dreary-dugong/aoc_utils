# aoc_utils
A collection of utility programs for automating portions of working on [Advent of Code](https://adventofcode.com) puzzles.

See the releases to download binaries. 

## aocfetch
Downloads your input for the current day's puzzles. See aocfetch/README.md for details and usage.

## aocsub
Submits an answer to the current day's puzzle and prints the result. See aocsub/README.md for details and usage. 

## aocex
Attempts to download the first example input on the page for the current day's puzzles. See aocex/README.md for details and usage.

## aocnew.sh
A bash script that automatically creates a new project for the day's puzzle's using the code in scaffold, then runs aocfetch to get an input, then opens up some windows for working on the puzzle in i3 workspaces. 

aocnew.sh is much less sophisticated than the other projects and is intended only for personal use, but can also serve as an example for others interested in automation. 

## scaffold
Some incomplete rust files used as a base for writing puzzle solutions. They constitute a rust program that takes a single input parameter for the input file, parses it, processes it, and prints the output. 


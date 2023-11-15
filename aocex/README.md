# AOCEX
A simple rust cli program that attempts to download the first example for a given [Advent of Code](https://adventofcode.com) puzzle

# Warnings
From adventofcode.com (in a comment in the html):

>Please be careful with automated requests; I'm not a massive company, and I can
>only take so much traffic.  Please be considerate so that everyone gets to play.

Please do not use this project to send too many unneccessary requests to AOC. 


Additionally, as per the home page:

>starting this December, please don't use AI to get on the global leaderboard.

Please do not use this project as part of a pipeline to solve puzzles automatically with AI to get on the global leaderboard. 

# Limitations
The program attempts to find the first example by looking for the first pre-formatted `<code>` element on the page. For the puzzles in recent years (2022, 2021), this is good enough
to find the example relatively consistently. It doesn't always work though, and you should read the page before running to check if it will. This is just a little utility that may or may not save
a few seconds of manually copying and pasting with the mouse. 

# Usage

The program was designed with sane defaults in mind (when running in December). To download today's example input and save it to example.txt, simply use

```aocex > example.txt```

or

```aocex -o example.txt```


To download the example for a different day, you can use the `--year` and `--day` flags. When run during December in the UTC-5 timezone (the timezone AOC uses),
The default year is the current year and the default day is the current day. When run during a different month, the default year is the previous year and
the default day is december 1st. Obviously, you cannot run it with a year that's too far in the past or in the future or a day greater than 31. 

```aocex --year 2022 --day 3 -o 2022day3ex.txt``` is equivalent to

```aocex -y 2022 -d 3 > 2022day3ex.txt```
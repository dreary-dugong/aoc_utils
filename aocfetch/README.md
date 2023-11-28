# aocfetch
A command line utility to download puzzle inputs for [Advent of Code](https://adventofcode.com)

This project was heavily inspired by [GreenLightning's aocdl project](https://github.com/GreenLightning/advent-of-code-downloader/blob/master/aocdl/main.go) but has some differences.
Notably, aocfetch lacks the wait functionality of aocdl but adds the ability to grab the user's session cookie directly from their firefox profile

## Warnings
From adventofcode.com (in a comment in the html):

>Please be careful with automated requests; I'm not a massive company, and I can
>only take so much traffic.  Please be considerate so that everyone gets to play.

Please do not use this project to send too many unneccessary requests to AOC. 


Additionally, as per the home page:

>starting this December, please don't use AI to get on the global leaderboard.

Please do not use this project as part of a pipeline to solve puzzles automatically with AI to get on the global leaderboard. 


Finally, this project has code to grab a user's session cookie from their firefox profile. 
Accomplishing this isn't necessarily difficult, but if this is the place you're first learning how do it, please use the knowledge responsibly. 
The world has enough token stealers already. 


## Usage

The program was designed with sane defaults in mind (when running in December). To download today's puzzle input and save it to input.txt, simply use

```aocfetch > input.txt```

or

```aocfetch -o input.txt```


To download the input for a different day, you can use the `--year` and `--day` flags. When run during December in the UTC-5 timezone (the timezone AOC uses),
The default year is the current year and the default day is the current day. When run during a different month, the default year is the previous year and
the default day is december 1st. Obviously, you cannot run it with a year that's too far in the past or in the future or a day greater than 31. 

```aocfetch --year 2015 --day 3 -o 2015day3.txt``` is equivalent to

```aocfetch -y 2015 -d 3 > 2015day3.txt```


By default, the program pulls the session cookie for `*.adventofcode.com` from the user's `default-release` firefox profile located in `~/.mozilla/firefox`.
If this is not preferable e.g. because you use another browser or because you use NixOS, there are other options to supply the cookie.

1. provide the session cookie directly with the `--cookie` flag e.g. `aocfetch --cookie xxxxxxxxxxxxxxxxxx` or `aocfetch -c xxxxxxxxxxxxxxxxxx`
2. provide the path to a file that contains the cookie with the `--file` flag e.g. `aocfetch --file ~/.mycookie.txt` or `aocfetch -f ~/.mycookie.txt`
3. provide an alternative folder for your firefox profile with the `--browser-folder` flag e.g. `aocfetch --browser-folder /etc/share/.mozilla` or `aocfetch -b /etc/share/.mozilla`


## FAQ
> Will you add support for $OTHER_BROWSER?

If someone files a PR, I'd be willing to merge it but I don't currently use chrome or any other browser beside firefox so I don't plan on doing it myself. 

> Will this work on Windows?

Maybe? I haven't tried it. The default cookie grabbing behavior almost certainly wouldn't work, though it might if you use `-b` and provide the correct windows Directory, but I see no reason why the rest of it wouldn't. Try at your own risk and let me know the results. 


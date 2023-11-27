#!/bin/bash

# if you know, you know.
set -e

# constants
scaffold_path="${HOME}/.aoc_utils/scaffold"
aoc_path="${HOME}/Documents/aoc2023"
layout_dir="${HOME}/.i3/layouts/aoc"

# what day of the month is it?
year=$(date +'%Y')
date=$(date +'%d')

today_dir="${aoc_path}/${date}"
level="1"
level_dir="${today_dir}/1"

# check if the first level has already been run (aka if we should be on level 2 or level 1)
if [ -d "${level_dir}" ]; then
    level_dir="${today_dir}/2"
    level="2"
fi
# check if the second level has already been run (in which case, exit before we fuck shit up)
if [ -d "${level_dir}" ]; then
    exit
fi

project_name="aoc${date}lvl${level}"

# aight we can do stuff now
# set up a new cargo project
mkdir -p "${level_dir}"
cd "${level_dir}"
cargo new "${project_name}"
cd "${project_name}"
cargo add clap --features derive
cargo add anyhow
cp "${scaffold_path}/main.rs" "./src/main.rs"
cp "${scaffold_path}/lib.rs" "./src/"
git add .
git commit -m "create project for day ${date} level ${level}"

# download input
aocfetch -o input.txt

# open windows
i3-msg "workspace 1; append_layout ${layout_dir}/workspace-1.json"
code . &

i3-msg "workspace 2; append_layout ${layout_dir}/workspace-2.json"
urxvt -cd "${level_dir}/${project_name}" &

i3-msg "workspace 3; append_layout ${layout_dir}/workspace-3.json"
firefox "https://adventofcode.com/${year}/${date}" &


exit
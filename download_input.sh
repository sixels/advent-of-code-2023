#!/bin/sh

source ./.env

day=$1

mkdir -p inputs

echo "downloading input for day $day"
if [ -f "./inputs/day$day" ]; then
	echo "nothing to do"
else
	curl --cookie "session=$session" "https://adventofcode.com/2023/day/$day/input" -o "./inputs/day$day" &&
		echo "downloaded the input file for day $day"
fi

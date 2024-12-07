#!/bin/sh

day=$1

if [ -z "$day" ]; then
	echo "Usage: $0 <day number>"
	echo "All available days:"
	ls -d day*
	exit 0
fi

cd ./day-"$day"
exec ./run.sh

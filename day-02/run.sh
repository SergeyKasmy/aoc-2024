#!/bin/sh

. ../util/run_solution.sh

run_solution \
	"Javascript / Typescript" \
	"js" \
	"" \
	"deno run --allow-read=.. main.ts ../input.txt"

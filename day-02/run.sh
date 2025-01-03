#!/bin/sh

. ../util/run_solution.sh

run_solution \
	"Javascript / Typescript" \
	"js" \
	"" \
	"deno run --allow-read=.. main.ts ../input.txt"

run_solution \
	"Rust" \
	"rust" \
	"cargo build --release" \
	"./target/release/day-02 ../input.txt"

hyperfine --shell=none --warmup=50 \
	'deno run --allow-read=. js/main.ts input.txt' \
	'rust/target/release/day-02 input.txt'

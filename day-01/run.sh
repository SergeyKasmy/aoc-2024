#!/bin/sh

. ../util/run_solution.sh

run_solution \
	"Javascript / Typescript" \
	"js" \
	"" \
	"deno run --allow-read=.. main.ts ../input.txt"

run_solution \
	"C#" \
	"c#" \
	"dotnet build -c Release" \
	"./bin/Release/net8.0/day-01 ../input.txt"

run_solution \
	"Rust" \
	"rust" \
	"cargo build --release" \
	"./target/release/day-01 ../input.txt"

hyperfine --shell=none --warmup=50 \
	'deno run --allow-read=. js/main.ts input.txt' \
	'c#/bin/Release/net8.0/day-01 input.txt' \
	'rust/target/release/day-01 input.txt'

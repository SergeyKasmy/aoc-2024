#!/bin/sh

run_solution() {
	name="$1"
	dir="$2"
	build_command="$3"
	run_command="$4"

	echo "-- $name"

	cd "./$dir"

	if [ -n "$build_command" ]; then
		echo " > $build_command"
		eval "$build_command"
	fi

	echo " > $run_command"
	eval "$run_command"

	echo
	cd ..
}

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

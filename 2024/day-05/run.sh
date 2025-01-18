#!/bin/sh

. ../util/run_solution.sh

run_solution \
	"Rust" \
	"rust" \
	"cargo build --release" \
	"./target/release/day-05 ../input.txt"

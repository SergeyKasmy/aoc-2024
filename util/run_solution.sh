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

	echo "\n > $run_command"
	eval "$run_command"

	echo
	cd ..
}

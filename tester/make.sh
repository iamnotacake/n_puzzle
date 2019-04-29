#!/bin/bash

mkdir -p inputs

for n in {3..12}; do
	name=$(printf "inputs/%.2d" $n)
	./npuzzle-gen.py $n > $name

	if grep unsolvable $name >/dev/null; then
		mv $name ${name}_unsolvable
	else
		mv $name ${name}_solvable
	fi
done

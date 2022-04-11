#!/bin/bash

for bench_workers in 1 2 3 4 5
do
	for kvs_workers in 1 2 3 4 5
	do
		target/release/bench-channels --bench-workers $bench_workers --workers $kvs_workers --run-for-seconds 30
	done
done

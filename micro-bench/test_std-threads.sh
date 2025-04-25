#!/bin/bash

iteration=
while [[ $# -gt 0 ]]; do
    case "$1" in
--iteration)
            iteration="$2"
        shift 2
            ;;
        *)
        echo "Unknown parameter: $1"
exit 1
;;
esac
done

if [[ -z "$iteration" ]]; then
echo "Error: --iteration argument is required."
exit 1
fi

if ! [[ "$iteration" =~ ^[0-9]+$ ]]; then
echo "Error: --iteration must be a positive integer."
exit 1
fi
rm logs/std-threads_*_iter${iteration}*.log
./target/release/micro-bench std-threads 2048 1 3000 2000 > logs/std-threads_nthread1_iter${iteration}.log 2>&1
./target/release/micro-bench std-threads 2048 5 3000 2000 > logs/std-threads_nthread5_iter${iteration}.log 2>&1
./target/release/micro-bench std-threads 2048 10 3000 2000 > logs/std-threads_nthread10_iter${iteration}.log 2>&1
./target/release/micro-bench std-threads 2048 15 3000 2000 > logs/std-threads_nthread15_iter${iteration}.log 2>&1
./target/release/micro-bench std-threads 2048 25 3000 2000 > logs/std-threads_nthread25_iter${iteration}.log 2>&1
./target/release/micro-bench std-threads 2048 30 3000 2000 > logs/std-threads_nthread30_iter${iteration}.log 2>&1

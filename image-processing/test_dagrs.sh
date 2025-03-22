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
rm logs/dagrs_*_iter${iteration}*.log
./target/release/image-processing dagrs 10 input_big > logs/dagrs_big_iter${iteration}.log 2>&1
./target/release/image-processing dagrs 10 input_mixed > logs/dagrs_mixed_iter${iteration}.log 2>&1
./target/release/image-processing dagrs 10 input_small > logs/dagrs_small_iter${iteration}.log 2>&1

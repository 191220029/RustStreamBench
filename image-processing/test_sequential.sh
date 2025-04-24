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
rm logs/sequential_*_iter${iteration}*.log
./target/release/image-processing sequential 4 input_big > logs/sequential_big_nthread4_iter${iteration}.log 2>&1
./target/release/image-processing sequential 4 input_mixed > logs/sequential_mixed_nthread4_iter${iteration}.log 2>&1
./target/release/image-processing sequential 4 input_small > logs/sequential_small_nthread4_iter${iteration}.log 2>&1
./target/release/image-processing sequential 8 input_big > logs/sequential_big_nthread8_iter${iteration}.log 2>&1
./target/release/image-processing sequential 8 input_mixed > logs/sequential_mixed_nthread8_iter${iteration}.log 2>&1
./target/release/image-processing sequential 8 input_small > logs/sequential_small_nthread8_iter${iteration}.log 2>&1
./target/release/image-processing sequential 16 input_big > logs/sequential_big_nthread16_iter${iteration}.log 2>&1
./target/release/image-processing sequential 16 input_mixed > logs/sequential_mixed_nthread16_iter${iteration}.log 2>&1
./target/release/image-processing sequential 16 input_small > logs/sequential_small_nthread16_iter${iteration}.log 2>&1
./target/release/image-processing sequential 32 input_big > logs/sequential_big_nthread32_iter${iteration}.log 2>&1
./target/release/image-processing sequential 32 input_mixed > logs/sequential_mixed_nthread32_iter${iteration}.log 2>&1
./target/release/image-processing sequential 32 input_small > logs/sequential_small_nthread32_iter${iteration}.log 2>&1

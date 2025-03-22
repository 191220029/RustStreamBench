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
rm logs/rust-ssp_*_iter${iteration}*.log
./target/release/image-processing rust-ssp 10 input_big > logs/rust-ssp_big_iter${iteration}.log 2>&1
./target/release/image-processing rust-ssp 10 input_mixed > logs/rust-ssp_mixed_iter${iteration}.log 2>&1
./target/release/image-processing rust-ssp 10 input_small > logs/rust-ssp_small_iter${iteration}.log 2>&1

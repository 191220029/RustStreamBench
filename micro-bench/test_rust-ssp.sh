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
./target/release/micro-bench rust-ssp 2048 4 3000 2000 > logs/rust-ssp_nthread4_iter${iteration}.log 2>&1
./target/release/micro-bench rust-ssp 2048 8 3000 2000 > logs/rust-ssp_nthread8_iter${iteration}.log 2>&1
./target/release/micro-bench rust-ssp 2048 16 3000 2000 > logs/rust-ssp_nthread16_iter${iteration}.log 2>&1
./target/release/micro-bench rust-ssp 2048 32 3000 2000 > logs/rust-ssp_nthread32_iter${iteration}.log 2>&1

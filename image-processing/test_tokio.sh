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
rm logs/tokio_*_iter${iteration}*.log
./target/release/image-processing tokio 1 input_big > logs/tokio_big_nthread1_iter${iteration}.log 2>&1
./target/release/image-processing tokio 1 input_mixed > logs/tokio_mixed_nthread1_iter${iteration}.log 2>&1
./target/release/image-processing tokio 1 input_small > logs/tokio_small_nthread1_iter${iteration}.log 2>&1
./target/release/image-processing tokio 5 input_big > logs/tokio_big_nthread5_iter${iteration}.log 2>&1
./target/release/image-processing tokio 5 input_mixed > logs/tokio_mixed_nthread5_iter${iteration}.log 2>&1
./target/release/image-processing tokio 5 input_small > logs/tokio_small_nthread5_iter${iteration}.log 2>&1
./target/release/image-processing tokio 10 input_big > logs/tokio_big_nthread10_iter${iteration}.log 2>&1
./target/release/image-processing tokio 10 input_mixed > logs/tokio_mixed_nthread10_iter${iteration}.log 2>&1
./target/release/image-processing tokio 10 input_small > logs/tokio_small_nthread10_iter${iteration}.log 2>&1
./target/release/image-processing tokio 15 input_big > logs/tokio_big_nthread15_iter${iteration}.log 2>&1
./target/release/image-processing tokio 15 input_mixed > logs/tokio_mixed_nthread15_iter${iteration}.log 2>&1
./target/release/image-processing tokio 15 input_small > logs/tokio_small_nthread15_iter${iteration}.log 2>&1
./target/release/image-processing tokio 25 input_big > logs/tokio_big_nthread25_iter${iteration}.log 2>&1
./target/release/image-processing tokio 25 input_mixed > logs/tokio_mixed_nthread25_iter${iteration}.log 2>&1
./target/release/image-processing tokio 25 input_small > logs/tokio_small_nthread25_iter${iteration}.log 2>&1
./target/release/image-processing tokio 30 input_big > logs/tokio_big_nthread30_iter${iteration}.log 2>&1
./target/release/image-processing tokio 30 input_mixed > logs/tokio_mixed_nthread30_iter${iteration}.log 2>&1
./target/release/image-processing tokio 30 input_small > logs/tokio_small_nthread30_iter${iteration}.log 2>&1

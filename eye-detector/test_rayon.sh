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
rm logs/rayon_*_iter${iteration}*.log
rm output_rayon.avi
./target/release/eye-detector rayon 10 ./inputs/mixed_15s.mp4 > logs/rayon_mixed_iter${iteration}.log 2>&1
./target/release/eye-detector rayon 10 ./inputs/one_face_15s.mp4 > logs/rayon_one_face_iter${iteration}.log 2>&1
./target/release/eye-detector rayon 10 ./inputs/several_faces_15s.mp4 > logs/rayon_several_faces_iter${iteration}.log 2>&1

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
rm logs/pipeliner_*_iter${iteration}*.log
rm output_pipeliner.avi
./target/release/eye-detector pipeliner 1 ./inputs/mixed_15s.mp4 > logs/pipeliner_mixed_nthread1_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 1 ./inputs/one_face_15s.mp4 > logs/pipeliner_one_face_nthread1_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 1 ./inputs/several_faces_15s.mp4 > logs/pipeliner_several_faces_nthread1_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 5 ./inputs/mixed_15s.mp4 > logs/pipeliner_mixed_nthread5_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 5 ./inputs/one_face_15s.mp4 > logs/pipeliner_one_face_nthread5_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 5 ./inputs/several_faces_15s.mp4 > logs/pipeliner_several_faces_nthread5_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 10 ./inputs/mixed_15s.mp4 > logs/pipeliner_mixed_nthread10_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 10 ./inputs/one_face_15s.mp4 > logs/pipeliner_one_face_nthread10_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 10 ./inputs/several_faces_15s.mp4 > logs/pipeliner_several_faces_nthread10_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 15 ./inputs/mixed_15s.mp4 > logs/pipeliner_mixed_nthread15_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 15 ./inputs/one_face_15s.mp4 > logs/pipeliner_one_face_nthread15_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 15 ./inputs/several_faces_15s.mp4 > logs/pipeliner_several_faces_nthread15_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 25 ./inputs/mixed_15s.mp4 > logs/pipeliner_mixed_nthread25_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 25 ./inputs/one_face_15s.mp4 > logs/pipeliner_one_face_nthread25_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 25 ./inputs/several_faces_15s.mp4 > logs/pipeliner_several_faces_nthread25_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 30 ./inputs/mixed_15s.mp4 > logs/pipeliner_mixed_nthread30_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 30 ./inputs/one_face_15s.mp4 > logs/pipeliner_one_face_nthread30_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 30 ./inputs/several_faces_15s.mp4 > logs/pipeliner_several_faces_nthread30_iter${iteration}.log 2>&1

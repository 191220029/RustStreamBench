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
./target/release/eye-detector pipeliner 4 ./inputs/mixed_15s.mp4 > logs/pipeliner_mixed_nthread4_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 4 ./inputs/one_face_15s.mp4 > logs/pipeliner_one_face_nthread4_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 4 ./inputs/several_faces_15s.mp4 > logs/pipeliner_several_faces_nthread4_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 8 ./inputs/mixed_15s.mp4 > logs/pipeliner_mixed_nthread8_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 8 ./inputs/one_face_15s.mp4 > logs/pipeliner_one_face_nthread8_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 8 ./inputs/several_faces_15s.mp4 > logs/pipeliner_several_faces_nthread8_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 16 ./inputs/mixed_15s.mp4 > logs/pipeliner_mixed_nthread16_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 16 ./inputs/one_face_15s.mp4 > logs/pipeliner_one_face_nthread16_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 16 ./inputs/several_faces_15s.mp4 > logs/pipeliner_several_faces_nthread16_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 32 ./inputs/mixed_15s.mp4 > logs/pipeliner_mixed_nthread32_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 32 ./inputs/one_face_15s.mp4 > logs/pipeliner_one_face_nthread32_iter${iteration}.log 2>&1
./target/release/eye-detector pipeliner 32 ./inputs/several_faces_15s.mp4 > logs/pipeliner_several_faces_nthread32_iter${iteration}.log 2>&1

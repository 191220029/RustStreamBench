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
rm output_dagrs.avi
./target/release/eye-detector dagrs 4 ./inputs/mixed_15s.mp4 > logs/dagrs_mixed_nthread4_iter${iteration}.log 2>&1
./target/release/eye-detector dagrs 4 ./inputs/one_face_15s.mp4 > logs/dagrs_one_face_nthread4_iter${iteration}.log 2>&1
./target/release/eye-detector dagrs 4 ./inputs/several_faces_15s.mp4 > logs/dagrs_several_faces_nthread4_iter${iteration}.log 2>&1
./target/release/eye-detector dagrs 8 ./inputs/mixed_15s.mp4 > logs/dagrs_mixed_nthread8_iter${iteration}.log 2>&1
./target/release/eye-detector dagrs 8 ./inputs/one_face_15s.mp4 > logs/dagrs_one_face_nthread8_iter${iteration}.log 2>&1
./target/release/eye-detector dagrs 8 ./inputs/several_faces_15s.mp4 > logs/dagrs_several_faces_nthread8_iter${iteration}.log 2>&1
./target/release/eye-detector dagrs 16 ./inputs/mixed_15s.mp4 > logs/dagrs_mixed_nthread16_iter${iteration}.log 2>&1
./target/release/eye-detector dagrs 16 ./inputs/one_face_15s.mp4 > logs/dagrs_one_face_nthread16_iter${iteration}.log 2>&1
./target/release/eye-detector dagrs 16 ./inputs/several_faces_15s.mp4 > logs/dagrs_several_faces_nthread16_iter${iteration}.log 2>&1
./target/release/eye-detector dagrs 32 ./inputs/mixed_15s.mp4 > logs/dagrs_mixed_nthread32_iter${iteration}.log 2>&1
./target/release/eye-detector dagrs 32 ./inputs/one_face_15s.mp4 > logs/dagrs_one_face_nthread32_iter${iteration}.log 2>&1
./target/release/eye-detector dagrs 32 ./inputs/several_faces_15s.mp4 > logs/dagrs_several_faces_nthread32_iter${iteration}.log 2>&1

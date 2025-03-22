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
rm -r workload/inputs
cp -r workload/backup workload/inputs
rm logs/sequential_*_iter${iteration}*.log
./target/release/bzip2 sequential 10 compress workload/inputs/avi_video.avi > "logs/sequential_compress_avi_video_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 10 compress workload/inputs/iso_file.iso > "logs/sequential_compress_iso_file_iter${iteration}.log" 2>&1 
./target/release/bzip2 sequential 10 compress workload/inputs/wiki_data > "logs/sequential_compress_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 sequential 10 compress workload/inputs/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/sequential_compress_jdk_iter${iteration}.log" 2>&1 

./target/release/bzip2 sequential 10 decompress workload/inputs/avi_video.avi.bz2 > "logs/sequential_decompress_avi_video_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 10 decompress workload/inputs/iso_file.iso.bz2 > "logs/sequential_decompress_iso_file_iter${iteration}.log" 2>&1 
./target/release/bzip2 sequential 10 decompress workload/inputs/wiki_data.bz2 > "logs/sequential_decompress_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 sequential 10 decompress workload/inputs/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/sequential_decompress_jdk_iter${iteration}.log" 2>&1

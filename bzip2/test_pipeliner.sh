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
rm -r workload/inputs/pipeliner
cp -r workload/backup workload/inputs/pipeliner
rm logs/pipeliner_*_iter${iteration}*.log
./target/release/bzip2 pipeliner 10 compress workload/inputs/pipeliner/avi_video.avi > "logs/pipeliner_compress_avi_video_iter${iteration}.log" 2>&1
./target/release/bzip2 pipeliner 10 compress workload/inputs/pipeliner/iso_file.iso > "logs/pipeliner_compress_iso_file_iter${iteration}.log" 2>&1 
./target/release/bzip2 pipeliner 10 compress workload/inputs/pipeliner/wiki_data > "logs/pipeliner_compress_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 pipeliner 10 compress workload/inputs/pipeliner/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/pipeliner_compress_jdk_iter${iteration}.log" 2>&1 

./target/release/bzip2 pipeliner 10 decompress workload/inputs/pipeliner/avi_video.avi.bz2 > "logs/pipeliner_decompress_avi_video_iter${iteration}.log" 2>&1
./target/release/bzip2 pipeliner 10 decompress workload/inputs/pipeliner/iso_file.iso.bz2 > "logs/pipeliner_decompress_iso_file_iter${iteration}.log" 2>&1 
./target/release/bzip2 pipeliner 10 decompress workload/inputs/pipeliner/wiki_data.bz2 > "logs/pipeliner_decompress_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 pipeliner 10 decompress workload/inputs/pipeliner/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/pipeliner_decompress_jdk_iter${iteration}.log" 2>&1

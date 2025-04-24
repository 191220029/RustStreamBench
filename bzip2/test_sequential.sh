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
rm -r workload/inputs/sequential
cp -r workload/backup workload/inputs/sequential
rm logs/sequential_*_iter${iteration}*.log
./target/release/bzip2 sequential 4 compress workload/inputs/sequential/avi_video.avi > "logs/sequential_compress_avi_video_nthread4_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 4 compress workload/inputs/sequential/iso_file.iso > "logs/sequential_compress_iso_file_nthread4_iter${iteration}.log" 2>&1 
./target/release/bzip2 sequential 4 compress workload/inputs/sequential/wiki_data > "logs/sequential_compress_nthread4_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 sequential 4 compress workload/inputs/sequential/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/sequential_compress_jdk_nthread4_iter${iteration}.log" 2>&1 

./target/release/bzip2 sequential 4 decompress workload/inputs/sequential/avi_video.avi.bz2 > "logs/sequential_decompress_avi_video_nthread4_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 4 decompress workload/inputs/sequential/iso_file.iso.bz2 > "logs/sequential_decompress_iso_file_nthread4_iter${iteration}.log" 2>&1 
./target/release/bzip2 sequential 4 decompress workload/inputs/sequential/wiki_data.bz2 > "logs/sequential_decompress_nthread4_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 sequential 4 decompress workload/inputs/sequential/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/sequential_decompress_jdk_nthread4_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 8 compress workload/inputs/sequential/avi_video.avi > "logs/sequential_compress_avi_video_nthread8_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 8 compress workload/inputs/sequential/iso_file.iso > "logs/sequential_compress_iso_file_nthread8_iter${iteration}.log" 2>&1 
./target/release/bzip2 sequential 8 compress workload/inputs/sequential/wiki_data > "logs/sequential_compress_nthread8_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 sequential 8 compress workload/inputs/sequential/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/sequential_compress_jdk_nthread8_iter${iteration}.log" 2>&1 

./target/release/bzip2 sequential 8 decompress workload/inputs/sequential/avi_video.avi.bz2 > "logs/sequential_decompress_avi_video_nthread8_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 8 decompress workload/inputs/sequential/iso_file.iso.bz2 > "logs/sequential_decompress_iso_file_nthread8_iter${iteration}.log" 2>&1 
./target/release/bzip2 sequential 8 decompress workload/inputs/sequential/wiki_data.bz2 > "logs/sequential_decompress_nthread8_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 sequential 8 decompress workload/inputs/sequential/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/sequential_decompress_jdk_nthread8_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 16 compress workload/inputs/sequential/avi_video.avi > "logs/sequential_compress_avi_video_nthread16_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 16 compress workload/inputs/sequential/iso_file.iso > "logs/sequential_compress_iso_file_nthread16_iter${iteration}.log" 2>&1 
./target/release/bzip2 sequential 16 compress workload/inputs/sequential/wiki_data > "logs/sequential_compress_nthread16_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 sequential 16 compress workload/inputs/sequential/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/sequential_compress_jdk_nthread16_iter${iteration}.log" 2>&1 

./target/release/bzip2 sequential 16 decompress workload/inputs/sequential/avi_video.avi.bz2 > "logs/sequential_decompress_avi_video_nthread16_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 16 decompress workload/inputs/sequential/iso_file.iso.bz2 > "logs/sequential_decompress_iso_file_nthread16_iter${iteration}.log" 2>&1 
./target/release/bzip2 sequential 16 decompress workload/inputs/sequential/wiki_data.bz2 > "logs/sequential_decompress_nthread16_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 sequential 16 decompress workload/inputs/sequential/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/sequential_decompress_jdk_nthread16_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 32 compress workload/inputs/sequential/avi_video.avi > "logs/sequential_compress_avi_video_nthread32_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 32 compress workload/inputs/sequential/iso_file.iso > "logs/sequential_compress_iso_file_nthread32_iter${iteration}.log" 2>&1 
./target/release/bzip2 sequential 32 compress workload/inputs/sequential/wiki_data > "logs/sequential_compress_nthread32_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 sequential 32 compress workload/inputs/sequential/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/sequential_compress_jdk_nthread32_iter${iteration}.log" 2>&1 

./target/release/bzip2 sequential 32 decompress workload/inputs/sequential/avi_video.avi.bz2 > "logs/sequential_decompress_avi_video_nthread32_iter${iteration}.log" 2>&1
./target/release/bzip2 sequential 32 decompress workload/inputs/sequential/iso_file.iso.bz2 > "logs/sequential_decompress_iso_file_nthread32_iter${iteration}.log" 2>&1 
./target/release/bzip2 sequential 32 decompress workload/inputs/sequential/wiki_data.bz2 > "logs/sequential_decompress_nthread32_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 sequential 32 decompress workload/inputs/sequential/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/sequential_decompress_jdk_nthread32_iter${iteration}.log" 2>&1

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
rm -r workload/inputs/tokio
cp -r workload/backup workload/inputs/tokio
rm logs/tokio_*_iter${iteration}*.log
./target/release/bzip2 tokio 1 compress workload/inputs/tokio/avi_video.avi > "logs/tokio_compress_avi_video_nthread1_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 1 compress workload/inputs/tokio/iso_file.iso > "logs/tokio_compress_iso_file_nthread1_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 1 compress workload/inputs/tokio/wiki_data > "logs/tokio_compress_nthread1_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 1 compress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/tokio_compress_jdk_nthread1_iter${iteration}.log" 2>&1 

./target/release/bzip2 tokio 1 decompress workload/inputs/tokio/avi_video.avi.bz2 > "logs/tokio_decompress_avi_video_nthread1_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 1 decompress workload/inputs/tokio/iso_file.iso.bz2 > "logs/tokio_decompress_iso_file_nthread1_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 1 decompress workload/inputs/tokio/wiki_data.bz2 > "logs/tokio_decompress_nthread1_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 1 decompress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/tokio_decompress_jdk_nthread1_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 5 compress workload/inputs/tokio/avi_video.avi > "logs/tokio_compress_avi_video_nthread5_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 5 compress workload/inputs/tokio/iso_file.iso > "logs/tokio_compress_iso_file_nthread5_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 5 compress workload/inputs/tokio/wiki_data > "logs/tokio_compress_nthread5_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 5 compress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/tokio_compress_jdk_nthread5_iter${iteration}.log" 2>&1 

./target/release/bzip2 tokio 5 decompress workload/inputs/tokio/avi_video.avi.bz2 > "logs/tokio_decompress_avi_video_nthread5_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 5 decompress workload/inputs/tokio/iso_file.iso.bz2 > "logs/tokio_decompress_iso_file_nthread5_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 5 decompress workload/inputs/tokio/wiki_data.bz2 > "logs/tokio_decompress_nthread5_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 5 decompress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/tokio_decompress_jdk_nthread5_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 10 compress workload/inputs/tokio/avi_video.avi > "logs/tokio_compress_avi_video_nthread10_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 10 compress workload/inputs/tokio/iso_file.iso > "logs/tokio_compress_iso_file_nthread10_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 10 compress workload/inputs/tokio/wiki_data > "logs/tokio_compress_nthread10_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 10 compress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/tokio_compress_jdk_nthread10_iter${iteration}.log" 2>&1 

./target/release/bzip2 tokio 10 decompress workload/inputs/tokio/avi_video.avi.bz2 > "logs/tokio_decompress_avi_video_nthread10_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 10 decompress workload/inputs/tokio/iso_file.iso.bz2 > "logs/tokio_decompress_iso_file_nthread10_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 10 decompress workload/inputs/tokio/wiki_data.bz2 > "logs/tokio_decompress_nthread10_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 10 decompress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/tokio_decompress_jdk_nthread10_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 15 compress workload/inputs/tokio/avi_video.avi > "logs/tokio_compress_avi_video_nthread15_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 15 compress workload/inputs/tokio/iso_file.iso > "logs/tokio_compress_iso_file_nthread15_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 15 compress workload/inputs/tokio/wiki_data > "logs/tokio_compress_nthread15_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 15 compress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/tokio_compress_jdk_nthread15_iter${iteration}.log" 2>&1 

./target/release/bzip2 tokio 15 decompress workload/inputs/tokio/avi_video.avi.bz2 > "logs/tokio_decompress_avi_video_nthread15_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 15 decompress workload/inputs/tokio/iso_file.iso.bz2 > "logs/tokio_decompress_iso_file_nthread15_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 15 decompress workload/inputs/tokio/wiki_data.bz2 > "logs/tokio_decompress_nthread15_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 15 decompress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/tokio_decompress_jdk_nthread15_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 25 compress workload/inputs/tokio/avi_video.avi > "logs/tokio_compress_avi_video_nthread25_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 25 compress workload/inputs/tokio/iso_file.iso > "logs/tokio_compress_iso_file_nthread25_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 25 compress workload/inputs/tokio/wiki_data > "logs/tokio_compress_nthread25_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 25 compress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/tokio_compress_jdk_nthread25_iter${iteration}.log" 2>&1 

./target/release/bzip2 tokio 25 decompress workload/inputs/tokio/avi_video.avi.bz2 > "logs/tokio_decompress_avi_video_nthread25_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 25 decompress workload/inputs/tokio/iso_file.iso.bz2 > "logs/tokio_decompress_iso_file_nthread25_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 25 decompress workload/inputs/tokio/wiki_data.bz2 > "logs/tokio_decompress_nthread25_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 25 decompress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/tokio_decompress_jdk_nthread25_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 30 compress workload/inputs/tokio/avi_video.avi > "logs/tokio_compress_avi_video_nthread30_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 30 compress workload/inputs/tokio/iso_file.iso > "logs/tokio_compress_iso_file_nthread30_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 30 compress workload/inputs/tokio/wiki_data > "logs/tokio_compress_nthread30_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 30 compress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/tokio_compress_jdk_nthread30_iter${iteration}.log" 2>&1 

./target/release/bzip2 tokio 30 decompress workload/inputs/tokio/avi_video.avi.bz2 > "logs/tokio_decompress_avi_video_nthread30_iter${iteration}.log" 2>&1
./target/release/bzip2 tokio 30 decompress workload/inputs/tokio/iso_file.iso.bz2 > "logs/tokio_decompress_iso_file_nthread30_iter${iteration}.log" 2>&1 
./target/release/bzip2 tokio 30 decompress workload/inputs/tokio/wiki_data.bz2 > "logs/tokio_decompress_nthread30_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 tokio 30 decompress workload/inputs/tokio/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/tokio_decompress_jdk_nthread30_iter${iteration}.log" 2>&1

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
rm -r workload/inputs/rayon
cp -r workload/backup workload/inputs/rayon
rm logs/rayon_*_iter${iteration}*.log
./target/release/bzip2 rayon 4 compress workload/inputs/rayon/avi_video.avi > "logs/rayon_compress_avi_video_nthread4_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 4 compress workload/inputs/rayon/iso_file.iso > "logs/rayon_compress_iso_file_nthread4_iter${iteration}.log" 2>&1 
./target/release/bzip2 rayon 4 compress workload/inputs/rayon/wiki_data > "logs/rayon_compress_nthread4_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 rayon 4 compress workload/inputs/rayon/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/rayon_compress_jdk_nthread4_iter${iteration}.log" 2>&1 

./target/release/bzip2 rayon 4 decompress workload/inputs/rayon/avi_video.avi.bz2 > "logs/rayon_decompress_avi_video_nthread4_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 4 decompress workload/inputs/rayon/iso_file.iso.bz2 > "logs/rayon_decompress_iso_file_nthread4_iter${iteration}.log" 2>&1 
./target/release/bzip2 rayon 4 decompress workload/inputs/rayon/wiki_data.bz2 > "logs/rayon_decompress_nthread4_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 rayon 4 decompress workload/inputs/rayon/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/rayon_decompress_jdk_nthread4_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 8 compress workload/inputs/rayon/avi_video.avi > "logs/rayon_compress_avi_video_nthread8_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 8 compress workload/inputs/rayon/iso_file.iso > "logs/rayon_compress_iso_file_nthread8_iter${iteration}.log" 2>&1 
./target/release/bzip2 rayon 8 compress workload/inputs/rayon/wiki_data > "logs/rayon_compress_nthread8_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 rayon 8 compress workload/inputs/rayon/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/rayon_compress_jdk_nthread8_iter${iteration}.log" 2>&1 

./target/release/bzip2 rayon 8 decompress workload/inputs/rayon/avi_video.avi.bz2 > "logs/rayon_decompress_avi_video_nthread8_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 8 decompress workload/inputs/rayon/iso_file.iso.bz2 > "logs/rayon_decompress_iso_file_nthread8_iter${iteration}.log" 2>&1 
./target/release/bzip2 rayon 8 decompress workload/inputs/rayon/wiki_data.bz2 > "logs/rayon_decompress_nthread8_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 rayon 8 decompress workload/inputs/rayon/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/rayon_decompress_jdk_nthread8_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 16 compress workload/inputs/rayon/avi_video.avi > "logs/rayon_compress_avi_video_nthread16_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 16 compress workload/inputs/rayon/iso_file.iso > "logs/rayon_compress_iso_file_nthread16_iter${iteration}.log" 2>&1 
./target/release/bzip2 rayon 16 compress workload/inputs/rayon/wiki_data > "logs/rayon_compress_nthread16_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 rayon 16 compress workload/inputs/rayon/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/rayon_compress_jdk_nthread16_iter${iteration}.log" 2>&1 

./target/release/bzip2 rayon 16 decompress workload/inputs/rayon/avi_video.avi.bz2 > "logs/rayon_decompress_avi_video_nthread16_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 16 decompress workload/inputs/rayon/iso_file.iso.bz2 > "logs/rayon_decompress_iso_file_nthread16_iter${iteration}.log" 2>&1 
./target/release/bzip2 rayon 16 decompress workload/inputs/rayon/wiki_data.bz2 > "logs/rayon_decompress_nthread16_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 rayon 16 decompress workload/inputs/rayon/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/rayon_decompress_jdk_nthread16_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 32 compress workload/inputs/rayon/avi_video.avi > "logs/rayon_compress_avi_video_nthread32_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 32 compress workload/inputs/rayon/iso_file.iso > "logs/rayon_compress_iso_file_nthread32_iter${iteration}.log" 2>&1 
./target/release/bzip2 rayon 32 compress workload/inputs/rayon/wiki_data > "logs/rayon_compress_nthread32_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 rayon 32 compress workload/inputs/rayon/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/rayon_compress_jdk_nthread32_iter${iteration}.log" 2>&1 

./target/release/bzip2 rayon 32 decompress workload/inputs/rayon/avi_video.avi.bz2 > "logs/rayon_decompress_avi_video_nthread32_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 32 decompress workload/inputs/rayon/iso_file.iso.bz2 > "logs/rayon_decompress_iso_file_nthread32_iter${iteration}.log" 2>&1 
./target/release/bzip2 rayon 32 decompress workload/inputs/rayon/wiki_data.bz2 > "logs/rayon_decompress_nthread32_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 rayon 32 decompress workload/inputs/rayon/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/rayon_decompress_jdk_nthread32_iter${iteration}.log" 2>&1

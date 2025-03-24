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
./target/release/bzip2 rayon 10 compress workload/inputs/rayon/avi_video.avi > "logs/rayon_compress_avi_video_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 10 compress workload/inputs/rayon/iso_file.iso > "logs/rayon_compress_iso_file_iter${iteration}.log" 2>&1 
./target/release/bzip2 rayon 10 compress workload/inputs/rayon/wiki_data > "logs/rayon_compress_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 rayon 10 compress workload/inputs/rayon/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/rayon_compress_jdk_iter${iteration}.log" 2>&1 

./target/release/bzip2 rayon 10 decompress workload/inputs/rayon/avi_video.avi.bz2 > "logs/rayon_decompress_avi_video_iter${iteration}.log" 2>&1
./target/release/bzip2 rayon 10 decompress workload/inputs/rayon/iso_file.iso.bz2 > "logs/rayon_decompress_iso_file_iter${iteration}.log" 2>&1 
./target/release/bzip2 rayon 10 decompress workload/inputs/rayon/wiki_data.bz2 > "logs/rayon_decompress_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 rayon 10 decompress workload/inputs/rayon/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/rayon_decompress_jdk_iter${iteration}.log" 2>&1

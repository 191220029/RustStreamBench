#!/bin/bash

# 解析命令行参数
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

# 验证参数
if [[ -z "$iteration" ]]; then
    echo "Error: --iteration argument is required."
    exit 1
fi

if ! [[ "$iteration" =~ ^[0-9]+$ ]]; then
    echo "Error: --iteration must be a positive integer."
    exit 1
fi

# 原有准备工作
cargo build --release
rm -r workload/inputs
cp -r workload/backup workload/inputs
rm logs/std_threads_*_iter${iteration}.log

# 运行命令（修改了所有日志路径）
./target/release/bzip2 std-threads 10 compress workload/inputs/avi_video.avi > "logs/std_threads_compress_avi_video_iter${iteration}.log" 2>&1
./target/release/bzip2 std-threads 10 compress workload/inputs/iso_file.iso > "logs/std_threads_compress_iso_file_iter${iteration}.log" 2>&1 
./target/release/bzip2 std-threads 10 compress workload/inputs/wiki_data > "logs/std_threads_compress_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 std-threads 10 compress workload/inputs/jdk-17.0.12_linux-x64_bin.tar.gz > "logs/std_threads_compress_jdk_iter${iteration}.log" 2>&1 

./target/release/bzip2 std-threads 10 decompress workload/inputs/avi_video.avi.bz2 > "logs/std_threads_decompress_avi_video_iter${iteration}.log" 2>&1
./target/release/bzip2 std-threads 10 decompress workload/inputs/iso_file.iso.bz2 > "logs/std_threads_decompress_iso_file_iter${iteration}.log" 2>&1 
./target/release/bzip2 std-threads 10 decompress workload/inputs/wiki_data.bz2 > "logs/std_threads_decompress_iter${iteration}.log_wiki_data" 2>&1 
./target/release/bzip2 std-threads 10 decompress workload/inputs/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > "logs/std_threads_decompress_jdk_iter${iteration}.log" 2>&1
cargo build --release
./target/release/bzip2 dagrs 10 compress /home/xiaolongfu/dagrs-perf/RustStreamBench/bzip2/workload/jdk-17.0.12_linux-x64_bin.tar.gz > dagrs_compress.log 2>&1 &
echo $! > dagrs_compress.pid
./target/release/bzip2 dagrs 10 decompress /home/xiaolongfu/dagrs-perf/RustStreamBench/bzip2/workload/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > dagrs_decompress.log 2>&1 &
echo $! > dagrs_decompress.pid

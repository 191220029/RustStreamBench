cargo build --release
rm -r workload/inputs
cp -r workload/backup workload/inputs
rm logs/tokio_*

./target/release/bzip2 tokio 10 compress workload/inputs/avi_video.avi > logs/tokio_compress_avi_video.log 2>&1
./target/release/bzip2 tokio 10 compress workload/inputs/iso_file.iso > logs/tokio_compress_iso_file.log 2>&1 
./target/release/bzip2 tokio 10 compress workload/inputs/wiki_data > logs/tokio_compress.log_wiki_data 2>&1 
./target/release/bzip2 tokio 10 compress workload/inputs/jdk-17.0.12_linux-x64_bin.tar.gz > logs/tokio_compress_jdk.log 2>&1 

./target/release/bzip2 tokio 10 decompress workload/inputs/avi_video.avi.bz2 > logs/tokio_decompress_avi_video.log 2>&1
./target/release/bzip2 tokio 10 decompress workload/inputs/iso_file.iso.bz2 > logs/tokio_decompress_iso_file.log 2>&1 
./target/release/bzip2 tokio 10 decompress workload/inputs/wiki_data.bz2 > logs/tokio_decompress.log_wiki_data 2>&1 
./target/release/bzip2 tokio 10 decompress workload/inputs/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > logs/tokio_decompress_jdk.log 2>&1 

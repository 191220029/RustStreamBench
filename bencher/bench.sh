cargo fmt && cargo build --release
rm ../bzip2/logs/* ../eye-detector/logs/* ../image-processing/logs/* ../micro-bench/logs/*
nohup ./target/release/bencher > bencher.log 2>&1 &
echo $! > bencher.pid
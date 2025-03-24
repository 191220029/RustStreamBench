cargo fmt && cargo build --release
nohup ./target/release/bencher > bencher.log 2>&1 &
echo $! > bencher.pid
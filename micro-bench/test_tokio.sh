cargo build --release
rm logs/tokio*

./target/release/micro-bench tokio 2048 10 3000 2000 > logs/tokio.log 2>&1

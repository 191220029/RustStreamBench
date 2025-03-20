cargo build --release
rm logs/dagrs*

./target/release/micro-bench dagrs 2048 10 3000 2000 > logs/dagrs.log 2>&1

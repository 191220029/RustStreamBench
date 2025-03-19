cargo build --release
rm logs/tokio_*

./target/release/image-processing tokio 10 input_big > logs/tokio_big.log 2>&1
./target/release/image-processing tokio 10 input_mixed > logs/tokio_mixed.log 2>&1 
./target/release/image-processing tokio 10 input_small > logs/tokio_small.log 2>&1 

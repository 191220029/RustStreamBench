cargo build --release
rm logs/dagrs_*

./target/release/image-processing dagrs 10 input_big > logs/dagrs_big.log 2>&1
./target/release/image-processing dagrs 10 input_mixed > logs/dagrs_mixed.log 2>&1 
./target/release/image-processing dagrs 10 input_small > logs/dagrs_small.log 2>&1 

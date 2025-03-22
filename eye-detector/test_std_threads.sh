cargo build --release
rm logs/std-threads*
rm output_std_threads.avi
./target/release/eye-detector std-threads 10 ./inputs/mixed_15s.mp4 > logs/std-threads.log 2>&1

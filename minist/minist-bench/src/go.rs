use std::process::Command;
use std::time::Instant;
use std::env;

pub fn run_go() {   
    let now = Instant::now(); // 程序起始时间
    env::set_var("RUST_LOG", "DEBUG");

    env_logger::init();

    let mut cmd = Command::new("go");
    cmd.current_dir("go-perf");
    cmd.arg("run");
    cmd.arg("sklearn.go");
    let output = cmd.output().expect("failed to run go");
    log::info!("{}", String::from_utf8(output.stdout).unwrap());

    let end = now.elapsed().as_secs();
    println!("time cost {:?}s", end); // 程序终止时间
}

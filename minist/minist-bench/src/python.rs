use std::process::Command;
use std::time::Instant;
use std::env;

pub fn run_python() {
    let now = Instant::now(); // 程序起始时间
    env::set_var("RUST_LOG", "DEBUG");

    env_logger::init();

    let mut cmd = Command::new("python");
    cmd.current_dir("..");
    cmd.arg("minist_parallel.py");
    let output = cmd.output().expect("failed to run minist_parallel.py");
    log::info!("{}", String::from_utf8(output.stdout).unwrap());
    log::info!("{}", String::from_utf8(output.stderr).unwrap());

    let end = now.elapsed().as_secs();
    println!("time cost {:?}s", end); // 程序终止时间
}

use std::env;

mod dagrs;
mod go;
mod liugi;
mod python;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!();
        panic!(
            "Correct usage: $ ./{:?} <runtime>",
            args[0]
        );
    }
    let run_mode = &args[1];

    match run_mode.as_str() {
        "go" => go::run_go(),
        "dagrs" => dagrs::run_dagrs(),
        "liugi" => liugi::run_liugi(),  
        "joblib" => python::run_python(),
        _ => println!("Invalid run_mode, use: dagrs, go, liugi, joblib"),
    }

    Ok(())
}

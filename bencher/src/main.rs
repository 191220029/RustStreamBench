use std::env;

use collect::collect_logs;

mod collect;
mod generate_test_script;
mod test_group;

const ITERATION: usize = 10;
const CSV_NAME: &str = "data.csv";
const NTHREADS: [usize; 6] = [1, 5, 10, 15, 25, 30];

fn main() {
    unsafe {
        env::set_var("RUST_LOG", "INFO");
    }
    env_logger::init();

    for test_group in generate_test_script::generate_test_script(NTHREADS.to_vec()) {
        test_group.run(ITERATION);
        collect_logs(
            test_group.pwd().join("logs").to_str().unwrap(),
            test_group.pwd().join(CSV_NAME).to_str().unwrap(),
        )
        .unwrap();
    }
}

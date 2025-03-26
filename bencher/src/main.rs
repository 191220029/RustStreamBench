use std::env;

use collect::collect_logs;

mod collect;
mod generate_test_script;
mod test_group;

const ITERATION: usize = 20;
const CSV_NAME: &str = "data.csv";

fn main() {
    unsafe {
        env::set_var("RUST_LOG", "INFO");
    }
    env_logger::init();

    for test_group in generate_test_script::generate_test_script(10) {
        test_group.run(ITERATION);
        collect_logs(
            test_group.pwd().join("logs").to_str().unwrap(),
            test_group.pwd().join(CSV_NAME).to_str().unwrap(),
        )
        .unwrap();
    }
}

use std::env;

use futures::StreamExt;

mod generate_test_script;
mod test_group;

const ITERATION: usize = 20;

fn main() {
    unsafe {
        env::set_var("RUST_LOG", "INFO");
    }
    env_logger::init();

    let mut handles = vec![];
    for test_group in generate_test_script::generate_test_script(10) {
        handles.append(&mut test_group.handles(ITERATION));
    }

    log::info!("{} tasks finished.", handles.len());
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let stream = futures::stream::iter(handles).buffer_unordered(20);
        let _ = stream.collect::<Vec<_>>().await;
    });
}

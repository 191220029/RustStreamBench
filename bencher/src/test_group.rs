use std::{path::PathBuf, process::Command};

use futures::StreamExt;
use tokio::task::JoinHandle;

pub struct TestGroup {
    pwd: PathBuf,
    scripts: Vec<PathBuf>,
}

impl TestGroup {
    pub fn new(pwd: PathBuf, scripts: Vec<PathBuf>) -> Self {
        if cfg!(unix) {
            for script in &scripts {
                let mut cmd = Command::new("chmod");
                cmd.args(["+x", script.to_str().unwrap()]);
                cmd.output().unwrap();
            }
        }

        Self { pwd, scripts }
    }

    pub fn pwd(&self) -> PathBuf {
        self.pwd.clone()
    }

    pub fn run(&self, iteration: usize) {
        // compile rust code
        let mut cmd = Command::new("cargo");
        cmd.args(["build", "--release"]);
        cmd.current_dir(self.pwd());
        cmd.output().unwrap();

        for i in 0..iteration {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let mut handles = vec![];
                for script in &self.scripts {
                    let pwd = self.pwd.clone();
                    let script = script.clone();
                    let task = tokio::task::spawn(async move {
                        log::info!(
                            "Running {} {}/{}",
                            script.to_str().unwrap(),
                            i + 1,
                            iteration
                        );
                        let mut cmd = Command::new(script.to_str().unwrap());
                        cmd.current_dir(pwd);
                        cmd.args(["--iteration", format!("{}", i).as_str()]);
                        cmd.output().unwrap();
                    });
                    handles.push(task);
                }
                let stream = futures::stream::iter(handles).buffer_unordered(20);
                let _ = stream.collect::<Vec<_>>().await;
            });
        }
    }
}

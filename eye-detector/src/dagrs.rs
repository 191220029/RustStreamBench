use std::{env, fs::File, io::Write, sync::Arc, time::SystemTime};

use dagrs::{
    async_trait::async_trait, Action, Content, DefaultNode, EnvVar, Graph, InChannels, Node,
    NodeId, NodeTable, OutChannels, Output,
};

const OUT_FILE: &str = "output_dagrs.avi";
const EYE: &str = "eye";
const FACE: &str = "face";
const WORKERS: &str = "workers";
const PRODUCERS: &str = "producers";
const READER: &str = "reader";
const WRITER: &str = "writer";

struct Reader;
#[dagrs::async_trait::async_trait]
impl Action for Reader {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {


        Output::empty()
    }
}


struct Eye(usize);
#[dagrs::async_trait::async_trait]
impl Action for Eye {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        

        Output::empty()
    }
}

struct Face(usize);
#[dagrs::async_trait::async_trait]
impl Action for Face {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        

        Output::empty()
    }
}


struct Writer;
#[dagrs::async_trait::async_trait]
impl Action for Writer {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {


        Output::empty()
    }
}

pub fn dagrs_eye_tracker(input_video: &String, nthreads: i32) -> opencv::Result<()> {
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    

    Ok(())
}

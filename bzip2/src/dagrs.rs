use std::{
    env,
    fs::File,
    io::{Read, Write},
    mem,
    sync::Arc, time::SystemTime,
};

use dagrs::{
    async_trait::async_trait, Action, Content, DefaultNode, EnvVar, Graph, InChannels, Node,
    NodeId, NodeTable, OutChannels, Output,
};

const COMPRESS_WORKERS: &str = "compress_workers";
const SLICER: &str = "slicer";
const REDUCER: &str = "reducer";
const BLOCK_SIZE: usize = 900000;
const COMPRESSED_FILE: &str = "compresssed_file";

struct Buffer {
    buffer: Vec<u8>,
    order: usize,
    size: usize,
}

impl Buffer {
    fn new(buffer: Vec<u8>, order: usize, size: usize) -> Self {
        Self {
            buffer,
            order,
            size,
        }
    }
}

struct Slicer {
    file_name: String,
    block_size: usize,
}

#[async_trait]
impl Action for Slicer {
    async fn run(
        &self,
        _: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let mut file = File::open(&self.file_name).expect("No file found.");
        let mut buffer_input = vec![];
        file.read_to_end(&mut buffer_input).unwrap();

        let mut pos_init: usize;
        let mut pos_end = 0;
        let mut bytes_left = buffer_input.len();
        let mut index = 0;
        let mut order: usize = 0;

        let workers: Vec<NodeId> = env.get(COMPRESS_WORKERS).unwrap();
        log::info!("{:?}", workers);

        while bytes_left > 0 {
            pos_init = pos_end;
            pos_end += if bytes_left < self.block_size {
                buffer_input.len() - pos_end
            } else {
                self.block_size
            };
            bytes_left -= pos_end - pos_init;

            let buffer_slice = buffer_input[pos_init..pos_end].to_vec();

            log::info!("slicer sending content to worker {:?}", workers[index]);
            out_channels
                .send_to(
                    &workers[index],
                    Content::new(Buffer::new(buffer_slice, order, 0)),
                )
                .await
                .unwrap();

            index += 1;
            order += 1;
            if index >= workers.len() {
                index = 0;
            }
        }

        log::info!("slicer closing output channel...");
        for id in workers {
            out_channels.close(&id);
        }

        Output::empty()
    }
}

struct Compress;
#[async_trait]
impl Action for Compress {
    async fn run(
        &self,
        in_channel: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let slicer_id: NodeId = env.get(SLICER).unwrap();
        let reducer_id: NodeId = env.get(REDUCER).unwrap();
        while let Ok(content) = in_channel.recv_from(&slicer_id).await {
            let buffer: Arc<Buffer> = content.into_inner().unwrap();
            let mut buffer = Buffer::new(buffer.buffer.clone(), buffer.order, buffer.size);

            log::info!(
                "compress worker receiving order {} with {} bytes.",
                buffer.order,
                buffer.buffer.len()
            );
            let mut buffer_output: Vec<u8> =
                vec![0; (buffer.buffer.len() as f64 * 1.01) as usize + 600];
            // computation
            unsafe {
                let mut bz_buffer: bzip2_sys::bz_stream = mem::zeroed();
                bzip2_sys::BZ2_bzCompressInit(&mut bz_buffer as *mut _, 9, 0, 30);

                bz_buffer.next_in = buffer.buffer.as_ptr() as *mut _;
                bz_buffer.avail_in = buffer.buffer.len() as _;
                bz_buffer.next_out = buffer_output.as_mut_ptr() as *mut _;
                bz_buffer.avail_out = buffer_output.len() as _;

                bzip2_sys::BZ2_bzCompress(&mut bz_buffer as *mut _, bzip2_sys::BZ_FINISH as _);
                bzip2_sys::BZ2_bzCompressEnd(&mut bz_buffer as *mut _);

                buffer.size = bz_buffer.total_out_lo32 as usize;
                buffer.buffer = buffer_output;
            }
            out_channels
                .send_to(&reducer_id, Content::new(buffer))
                .await
                .unwrap();
        }

        out_channels.close(&reducer_id);

        Output::empty()
    }
}
struct DeCompress;
#[async_trait]
impl Action for DeCompress {
    async fn run(
        &self,
        in_channel: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let slicer_id: NodeId = env.get(SLICER).unwrap();
        let reducer_id: NodeId = env.get(REDUCER).unwrap();
        while let Ok(content) = in_channel.recv_from(&slicer_id).await {
            let buffer: Arc<Buffer> = content.into_inner().unwrap();
            let mut buffer = Buffer::new(buffer.buffer.clone(), buffer.order, buffer.size);

            log::info!(
                "compress worker receiving order {} with {} bytes.",
                buffer.order,
                buffer.buffer.len()
            );
            let mut buffer_output: Vec<u8> = vec![0; BLOCK_SIZE];
            // computation
            unsafe {
                let mut bz_buffer: bzip2_sys::bz_stream = mem::zeroed();
                bzip2_sys::BZ2_bzDecompressInit(&mut bz_buffer as *mut _, 0, 0);

                bz_buffer.next_in = buffer.buffer.as_ptr() as *mut _;
                bz_buffer.avail_in = buffer.buffer.len() as _;
                bz_buffer.next_out = buffer_output.as_mut_ptr() as *mut _;
                bz_buffer.avail_out = buffer_output.len() as _;

                bzip2_sys::BZ2_bzDecompress(&mut bz_buffer as *mut _);
                bzip2_sys::BZ2_bzDecompressEnd(&mut bz_buffer as *mut _);

                buffer.size = bz_buffer.total_out_lo32 as usize;
                buffer.buffer = buffer_output;
            }
            out_channels
                .send_to(&reducer_id, Content::new(buffer))
                .await
                .unwrap();
        }

        out_channels.close(&reducer_id);

        Output::empty()
    }
}

struct Reduce;
#[async_trait]
impl Action for Reduce {
    async fn run(
        &self,
        in_channel: &mut InChannels,
        _: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let workers: Vec<NodeId> = env.get(COMPRESS_WORKERS).unwrap();
        let mut outputs = vec![];
        for slicer_id in workers {
            while let Ok(content) = in_channel.recv_from(&slicer_id).await {
                let buffer: Arc<Buffer> = content.into_inner().unwrap();
                log::info!(
                    "Reducer received order {} with {} bytes",
                    buffer.order,
                    buffer.buffer.len()
                );
                outputs.push(buffer);
            }
        }

        outputs.sort_by_key(|x| x.order);
        let mut buffer_output: Vec<u8> = vec![];
        for content in outputs {
            buffer_output.extend(&content.buffer[0..content.size as usize]);
        }

        let compressed_file_name: String = env.get(COMPRESSED_FILE).unwrap();
        log::info!(
            "{} bytes writen to {}",
            buffer_output.len(),
            compressed_file_name
        );
        let mut buf_write = File::create(compressed_file_name).unwrap();
        // write compressed data to file
        buf_write.write_all(&buffer_output).unwrap();

        Output::empty()
    }
}

pub fn run_dagrs(threads: usize, file_action: &str, file_name: &str) {
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    
    let start = SystemTime::now();  

    let mut node_table = NodeTable::new();

    let mut graph = Graph::new();

    let reducer = DefaultNode::with_action(format!("reducer").to_owned(), Reduce, &mut node_table);
    let reducer_id = reducer.id();
    graph.add_node(reducer);

    if file_action == "compress" {
        let compressed_file_name = file_name.to_owned() + &".bz2";

        let mut worker_id = vec![];
        let slicer = DefaultNode::with_action(
            format!("Slicer"),
            Slicer {
                file_name: file_name.to_owned(),
                block_size: BLOCK_SIZE,
            },
            &mut node_table,
        );
        let slicer_id = slicer.id();
        graph.add_node(slicer);

        for i in 0..threads {
            let node = DefaultNode::with_action(format!("worker_{}", i), Compress, &mut node_table);
            let node_id = node.id();
            worker_id.push(node_id);
            graph.add_node(node);
            graph.add_edge(node_id, vec![reducer_id]);
        }
        graph.add_edge(slicer_id, worker_id.clone());

        let mut env = EnvVar::new(node_table);
        env.set(COMPRESSED_FILE, compressed_file_name);
        env.set(COMPRESS_WORKERS, worker_id);
        env.set(SLICER, slicer_id);
        env.set(REDUCER, reducer_id);
        graph.set_env(env);

        graph.start().unwrap();
    } else if file_action == "decompress" {
        // creating the decompressed file
        let decompressed_file_name = &file_name.to_owned()[..file_name.len() - 4];
        let mut worker_id = vec![];

        let slicer = DefaultNode::with_action(
            "slicer".to_owned(),
            Slicer {
                file_name: file_name.to_owned(),
                block_size: BLOCK_SIZE,
            },
            &mut node_table,
        );
        let slicer_id = slicer.id();
        graph.add_node(slicer);
        for i in 0..threads {
            let node = DefaultNode::with_action(format!("worker_{}", i), DeCompress, &mut node_table);
            let node_id = node.id();
            worker_id.push(node_id);
            graph.add_node(node);
            graph.add_edge(node_id, vec![reducer_id]);
        }
        graph.add_edge(slicer_id, worker_id.clone());

        let mut env = EnvVar::new(node_table);
        env.set(COMPRESSED_FILE, decompressed_file_name.to_owned());
        env.set(COMPRESS_WORKERS, worker_id);
        env.set(SLICER, slicer_id);
        env.set(REDUCER, reducer_id);
        graph.set_env(env);

        graph.start().unwrap();
    }

    let system_duration = start.elapsed().expect("Failed to get render time?");
        let in_sec =
            system_duration.as_secs() as f64 + system_duration.subsec_nanos() as f64 * 1e-9;
        println!("Execution time: {} sec", in_sec);
}

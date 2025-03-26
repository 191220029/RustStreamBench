use std::{env, fs::File, io::Write, sync::Arc, time::SystemTime};

use dagrs::{
    async_trait::async_trait, Action, Content, DefaultNode, EnvVar, Graph, InChannels, Node,
    NodeId, NodeTable, OutChannels, Output,
};

const ITER_SIZE_1: &str = "iter_size_1";
const ITER_SIZE_2: &str = "iter_size_2";
const GENERATOR: &str = "reader";
const FRACTAL_A: &str = "fractal_a";
const FRACTAL_B: &str = "fractal_b";
const WORKERS: &str = "workers";
const PRODUCERS: &str = "producers";
const WRITER: &str = "writer";

#[derive(Clone)]
struct Tcontent {
    size: usize,
    line: i64,
    line_buffer: Vec<u8>,
    a_buffer: Vec<f64>,
    b_buffer: Vec<f64>,
    k_buffer: Vec<i32>,
}

struct Generator(usize);
#[async_trait]
impl Action for Generator {
    async fn run(
        &self,
        _: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let workers: Vec<NodeId> = env.get(WORKERS).unwrap();

        let size = self.0;
        let mut index = 0;

        for i in 0..size {
            let content = Tcontent {
                /*order: i as u64,*/
                size,
                line: i as i64,
                line_buffer: vec![0; size],
                a_buffer: vec![0.0; size],
                b_buffer: vec![0.0; size],
                k_buffer: vec![0; size],
            };

            log::info!("Generator sending content to worker {:?}", workers[index]);
            out_channels
                .send_to(&workers[index], Content::new(content))
                .await
                .unwrap();

            index += 1;
            if index >= workers.len() {
                index = 0;
            }
        }

        for worker in workers {
            out_channels.close(&worker);
        }

        Output::empty()
    }
}

struct FractalA(usize);
#[async_trait]
impl Action for FractalA {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let reader_id: NodeId = env.get(GENERATOR).unwrap();
        let fractal_b: NodeId = env.get(&format!("{}_{}", FRACTAL_B, self.0)).unwrap();
        let iter_size1: i32 = env.get(ITER_SIZE_1).unwrap();

        while let Ok(content) = in_channels.recv_from(&reader_id).await {
            log::info!("FractalA({}) processing...", self.0);
            let content: &Tcontent = content.get().unwrap();
            let mut content = content.clone();

            // computation
            let init_a = -2.125 as f64;
            let init_b = -1.5 as f64;
            let range = 3.0 as f64;
            let step = range / (content.size as f64);

            let im = init_b + (step * (content.line as f64));

            for j in 0..content.size {
                let mut a = init_a + step * j as f64;
                let cr = a;

                let mut b = im;
                let mut k = 0;

                for ii in 0..iter_size1 {
                    let a2 = a * a;
                    let b2 = b * b;
                    if (a2 + b2) > 4.0 {
                        break;
                    }
                    b = 2.0 * a * b + im;
                    a = a2 - b2 + cr;
                    k = ii;
                }
                content.a_buffer[j] = a;
                content.b_buffer[j] = b;
                content.k_buffer[j] = k;
            }

            out_channels
                .send_to(&fractal_b, Content::new(content))
                .await
                .unwrap();
        }

        out_channels.close(&fractal_b);

        Output::empty()
    }
}

struct FractalB(usize);
#[async_trait]
impl Action for FractalB {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let fractal_a_id: NodeId = env.get(&format!("{}_{}", FRACTAL_A, self.0)).unwrap();
        let writer_id: NodeId = env.get(WRITER).unwrap();
        let iter_size1: i32 = env.get(ITER_SIZE_1).unwrap();
        let iter_size2: i32 = env.get(ITER_SIZE_2).unwrap();

        while let Ok(content) = in_channels.recv_from(&fractal_a_id).await {
            log::info!("FractalB({}) processing...", self.0);
            let content: &Tcontent = content.get().unwrap();
            let mut content = content.clone();

            // computation
            let init_a = -2.125 as f64;
            let init_b = -1.5 as f64;
            let range = 3.0 as f64;
            let step = range / (content.size as f64);

            let im = init_b + (step * (content.line as f64));

            for j in 0..content.size {
                let cr = init_a + step * j as f64;
                if content.k_buffer[j] == iter_size1 - 1 {
                    for ii in iter_size1..iter_size1 + iter_size2 {
                        let a2 = content.a_buffer[j] * content.a_buffer[j];
                        let b2 = content.b_buffer[j] * content.b_buffer[j];
                        if (a2 + b2) > 4.0 {
                            break;
                        }
                        content.b_buffer[j] = 2.0 * content.a_buffer[j] * content.b_buffer[j] + im;
                        content.a_buffer[j] = a2 - b2 + cr;
                        content.k_buffer[j] = ii;
                    }
                }
                content.line_buffer[j] = (255 as f64
                    - ((content.k_buffer[j] as f64) * 255 as f64
                        / ((iter_size1 + iter_size2) as f64)))
                    as u8;
            }

            out_channels
                .send_to(&writer_id, Content::new(content))
                .await
                .unwrap();
        }

        out_channels.close(&writer_id);

        Output::empty()
    }
}

struct Writer;
#[async_trait]
impl Action for Writer {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        _: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let mut producers: Vec<NodeId> = env.get(PRODUCERS).unwrap();
        let mut collection = vec![];

        let mut index = 0;

        while producers.len() > 0 {
            if let Ok(content) = in_channels.recv_from(&producers[index]).await {
                log::info!("Writer received a packet");
                let content: &Tcontent = content.get().unwrap();
                collection.push(content.clone());
                index += 1;
            } else {
                producers.remove(index);
            }

            if index == producers.len() {
                index = 0;
            }
        }

        let mut m = vec![];
        for line in collection {
            m.extend(line.line_buffer);
        }
        let mut buffer = File::create("result_dagrs.txt").unwrap();
        buffer.write_all(&m).unwrap();

        Output::empty()
    }
}

pub fn dagrs_pipeline(size: usize, threads: usize, iter_size1: i32, iter_size2: i32) {
    let start = SystemTime::now();
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let mut graph = Graph::new();
    let mut node_table = NodeTable::default();

    let mut node_ids = vec![];
    let mut workers = vec![];
    let mut producers = vec![];

    let generator =
        DefaultNode::with_action(format!("generator"), Generator(size), &mut node_table);
    let generator_id = generator.id();
    graph.add_node(generator);

    let writer = DefaultNode::with_action(format!("writer"), Writer, &mut node_table);
    let writer_id = writer.id();
    graph.add_node(writer);

    for i in 0..threads {
        let fractal_a_name = format!("{}_{}", FRACTAL_A, i);
        let fractal_a =
            DefaultNode::with_action(fractal_a_name.clone(), FractalA(i), &mut node_table);
        let fractal_a_id = fractal_a.id();
        graph.add_node(fractal_a);
        workers.push(fractal_a_id);
        node_ids.push((fractal_a_name, fractal_a_id));

        let fractal_b_name = format!("{}_{}", FRACTAL_B, i);
        let fractal_b =
            DefaultNode::with_action(fractal_b_name.clone(), FractalB(i), &mut node_table);
        let fractal_b_id = fractal_b.id();
        graph.add_node(fractal_b);
        producers.push(fractal_b_id);
        node_ids.push((fractal_b_name, fractal_b_id));
        graph.add_edge(fractal_b_id, vec![writer_id]);

        graph.add_edge(fractal_a_id, vec![fractal_b_id]);
    }

    graph.add_edge(generator_id, workers.clone());

    let mut env = EnvVar::new(node_table);
    env.set(GENERATOR, generator_id);
    env.set(WRITER, writer_id);
    env.set(ITER_SIZE_1, iter_size1);
    env.set(ITER_SIZE_2, iter_size2);
    env.set(WORKERS, workers);
    env.set(PRODUCERS, producers);
    for (k, v) in node_ids {
        env.set(&k, v);
    }

    graph.set_env(env);
    graph.start().unwrap();

    let system_duration = start.elapsed().expect("Failed to get render time?");
    let in_sec = system_duration.as_secs() as f64 + system_duration.subsec_nanos() as f64 * 1e-9;
    println!("Execution time: {} sec", in_sec);
}

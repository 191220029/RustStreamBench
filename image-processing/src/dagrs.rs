use std::{env, sync::Arc, time::SystemTime};

use dagrs::{
    Action, Content, DefaultNode, EnvVar, Graph, InChannels, Node, NodeId, NodeTable, OutChannels,
    Output,
};
use raster::{filter, Image};

const SAT: f32 = 0.2;
const GAMMA: f32 = 0.2;
const WORKERS: &str = "workers";
const IMAGE_DIR: &str = "image_dir";

const SATURATOR: &str = "saturator";
const EMBOSSER: &str = "embosser";
const GAMMAER: &str = "gammaer";
const SHARPENER: &str = "sharpener";
const GRAY_SCALER: &str = "gray_scaler";

struct Split;
#[dagrs::async_trait::async_trait]
impl Action for Split {
    async fn run(
        &self,
        _: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let workers: Vec<NodeId> = env.get(WORKERS).unwrap();
        log::info!("Spliter detecting workers: {:?}", workers);

        let image_dir: String = env.get(IMAGE_DIR).unwrap();

        let dir_entries = std::fs::read_dir(image_dir);

        let mut index = 0;
        for entry in dir_entries.unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.extension().is_none() {
                continue;
            }

            log::info!("Spliter Sending {:?} to {:?}", path, workers[index]);
            out_channels
                .send_to(
                    &workers[index],
                    Content::new(raster::open(path.to_str().unwrap()).unwrap()),
                )
                .await
                .unwrap();

            index += 1;
            if index >= workers.len() {
                index = 0;
            }
        }

        log::info!("Spliter closing channels...");
        for id in workers {
            out_channels.close(&id);
        }

        Output::empty()
    }
}

struct Saturator {
    split_id: NodeId,
    order: usize,
}
#[dagrs::async_trait::async_trait]
impl Action for Saturator {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let dst_id: NodeId = env.get(&format!("{}_{}", EMBOSSER, self.order)).unwrap();

        while let Ok(content) = in_channels.recv_from(&self.split_id).await {
            let image: &Image = content.get().unwrap();
            let mut image = image.clone();
            filter::saturation(&mut image, SAT).unwrap();

            log::info!("Saturator process complete.");
            out_channels
                .send_to(&dst_id, Content::new(image))
                .await
                .unwrap();
        }
        out_channels.close(&dst_id);

        Output::empty()
    }
}

struct Embosser {
    order: usize,
}
#[dagrs::async_trait::async_trait]
impl Action for Embosser {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let src_id: NodeId = env.get(&format!("{}_{}", SATURATOR, self.order)).unwrap();
        let dst_id: NodeId = env.get(&format!("{}_{}", GAMMAER, self.order)).unwrap();

        while let Ok(image) = in_channels.recv_from(&src_id).await {
            let image: &Image = image.get().unwrap();
            let mut image = image.clone();

            filter::emboss(&mut image).unwrap();
            out_channels
                .send_to(&dst_id, Content::new(image))
                .await
                .unwrap();

            log::info!("Embosser process complete.");
        }
        out_channels.close(&dst_id);

        Output::empty()
    }
}

struct Gammaer {
    order: usize,
}
#[dagrs::async_trait::async_trait]
impl Action for Gammaer {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let src_id: NodeId = env.get(&format!("{}_{}", EMBOSSER, self.order)).unwrap();
        let dst_id: NodeId = env.get(&format!("{}_{}", SHARPENER, self.order)).unwrap();

        while let Ok(image) = in_channels.recv_from(&src_id).await {
            let image: &Image = image.get().unwrap();
            let mut image = image.clone();

            filter::gamma(&mut image, GAMMA).unwrap();
            out_channels
                .send_to(&dst_id, Content::new(image))
                .await
                .unwrap();

            log::info!("Gammaer process complete.");
        }
        out_channels.close(&dst_id);

        Output::empty()
    }
}

struct Sharpener {
    order: usize,
}
#[dagrs::async_trait::async_trait]
impl Action for Sharpener {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let src_id: NodeId = env.get(&format!("{}_{}", GAMMAER, self.order)).unwrap();
        let dst_id: NodeId = env.get(&format!("{}_{}", GRAY_SCALER, self.order)).unwrap();

        while let Ok(image) = in_channels.recv_from(&src_id).await {
            let image: &Image = image.get().unwrap();
            let mut image = image.clone();

            filter::sharpen(&mut image).unwrap();
            out_channels
                .send_to(&dst_id, Content::new(image))
                .await
                .unwrap();

            log::info!("Sharpener process complete.");
        }
        out_channels.close(&dst_id);

        Output::empty()
    }
}

struct GrayScaler {
    order: usize,
}
#[dagrs::async_trait::async_trait]
impl Action for GrayScaler {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        _: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let src_id: NodeId = env.get(&format!("{}_{}", SHARPENER, self.order)).unwrap();
        while let Ok(image) = in_channels.recv_from(&src_id).await {
            let image: &Image = image.get().unwrap();
            let mut image = image.clone();

            filter::grayscale(&mut image).unwrap();
            // out_channels
            //     .send_to(&self.reducer_id, Content::new(image))
            //     .await
            //     .unwrap();

            log::info!("GrayScaler process complete.");
        }

        Output::empty()
    }
}

// struct Reducer;
// #[dagrs::async_trait::async_trait]
// impl Action for Reducer {
//     async fn run(
//         &self,
//         in_channels: &mut InChannels,
//         _: &mut OutChannels,
//         _: Arc<EnvVar>,
//     ) -> Output {
//         let mut cnt = 0;
//         for _ in in_channels
//             .map(|content| {
//                 let image = content.unwrap();
//                 let image: &Image = image.get().unwrap();
//                 image.clone()
//             })
//             .await
//         {
//             cnt += 1;
//         }
//         log::info!("Reducer received {} processed images.", cnt);
//         Output::empty()
//     }
// }

pub fn run_dagrs(dir_name: &str, threads: usize) {
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let start = SystemTime::now();

    let mut node_table = NodeTable::default();
    let mut graph = Graph::new();

    let spliter = DefaultNode::with_action(format!("spliter"), Split, &mut node_table);
    let spliter_id = spliter.id();
    graph.add_node(spliter);
    // let reducer = DefaultNode::with_action(format!("reducer"), Reducer, &mut node_table);
    // let reducer_id: NodeId = reducer.id();
    // graph.add_node(reducer);

    let mut worker_id = vec![];
    let mut ids = vec![];

    for i in 0..threads {
        let gray_scaler = DefaultNode::with_action(
            format!("{}_{}", GRAY_SCALER, i),
            GrayScaler { order: i },
            &mut node_table,
        );
        let gray_scaler_id = gray_scaler.id();
        graph.add_node(gray_scaler);
        // graph.add_edge(gray_scaler_id, vec![reducer_id]);
        ids.push((format!("{}_{}", GRAY_SCALER, i), gray_scaler_id));

        let sharpener = DefaultNode::with_action(
            format!("{}_{}", SHARPENER, i),
            Sharpener { order: i },
            &mut node_table,
        );
        let sharpener_id = sharpener.id();
        graph.add_node(sharpener);
        graph.add_edge(sharpener_id, vec![gray_scaler_id]);
        ids.push((format!("{}_{}", SHARPENER, i), sharpener_id));

        let gammaer = DefaultNode::with_action(
            format!("{}_{}", GAMMAER, i),
            Gammaer { order: i },
            &mut node_table,
        );
        let gammaer_id = gammaer.id();
        graph.add_node(gammaer);
        graph.add_edge(gammaer_id, vec![sharpener_id]);
        ids.push((format!("{}_{}", GAMMAER, i), gammaer_id));

        let embosser = DefaultNode::with_action(
            format!("{}_{}", EMBOSSER, i),
            Embosser { order: i },
            &mut node_table,
        );
        let embosser_id = embosser.id();
        graph.add_node(embosser);
        graph.add_edge(embosser_id, vec![gammaer_id]);
        ids.push((format!("{}_{}", EMBOSSER, i), embosser_id));

        let saturator = DefaultNode::with_action(
            format!("{}_{}", SATURATOR, i),
            Saturator {
                split_id: spliter_id,
                order: i,
            },
            &mut node_table,
        );
        let saturator_id = saturator.id();
        graph.add_node(saturator);
        graph.add_edge(saturator_id, vec![embosser_id]);
        ids.push((format!("{}_{}", SATURATOR, i), saturator_id));
        worker_id.push(saturator_id);
    }

    graph.add_edge(spliter_id, worker_id.clone());

    let mut env = EnvVar::new(node_table);
    env.set(IMAGE_DIR, dir_name.to_string());
    env.set(WORKERS, worker_id);
    for (name, id) in ids {
        env.set(&name, id);
    }
    graph.set_env(env);

    graph.start().unwrap();

    let system_duration = start.elapsed().expect("Failed to get render time?");
    let in_sec = system_duration.as_secs() as f64 + system_duration.subsec_nanos() as f64 * 1e-9;
    println!("Execution time: {} sec", in_sec);
}

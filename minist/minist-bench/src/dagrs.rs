use std::{env, process::Command, sync::Arc, time::Instant};

use dagrs::{
    async_trait::async_trait, Action, Content, DefaultNode, EnvVar, Graph,
    InChannels, Node, NodeId, NodeTable, OutChannels, Output,
};

const ENV_DATA_SRC: &str = "data_src";

// #[global_allocator]
// static ALLOC: dhat::Alloc = dhat::Alloc;

struct NodeAction {
    class: usize,
    root_id: NodeId,
}

#[async_trait]
impl Action for NodeAction {
    async fn run(
        &self,
        _: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        log::debug!("Class {} training...", self.class);

        let data_src: &&str = env.get_ref(ENV_DATA_SRC).unwrap();

        let mut cmd = Command::new("python");
        cmd.args(vec![
            "-B".to_string(),
            "../minist_i.py".to_string(),
            format!("{}", data_src),
            format!("{}", self.class),
        ]);

        let output = cmd.output().unwrap();
        let std = output.stdout;
        out_channels
            .send_to(&self.root_id, Content::new(std))
            .await
            .unwrap();
        Output::empty()
    }
}

impl NodeAction {
    fn new(class: usize, root_id: NodeId) -> Self {
        Self { class, root_id }
    }
}

struct RootAction;
#[async_trait]
impl Action for RootAction {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        _: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let data_src: &&str = env.get_ref(ENV_DATA_SRC).unwrap();
        let mut thetas: Vec<String> = in_channels
            .map(|content| {
                let content = content.unwrap();
                let theta = content.get::<Vec<u8>>().unwrap();
                String::from_utf8(theta.clone()).unwrap()
            })
            .await;

        let mut args = vec![
            "-B".to_string(),
            "../minist_root.py".to_string(),
            format!("{}", data_src),
        ];

        args.append(&mut thetas);

        let mut cmd = Command::new("python");
        cmd.args(args);
        let output = cmd.output().unwrap().stdout;
        Output::new(output)
    }
}

pub fn run_dagrs() {
    // let _profiler = dhat::Profiler::new_heap();
    let now = Instant::now(); // 程序起始时间
    env::set_var("RUST_LOG", "DEBUG");

    env_logger::init();

    let mut node_table = NodeTable::new();
    let root_node = DefaultNode::with_action("root".to_string(), RootAction, &mut node_table);
    let root_id = root_node.id();

    let mut dag = Graph::new();
    dag.add_node(root_node);

    vec![
        "node0", "node1", "node2", "node3", "node4", "node5", "node6", "node7", "node8", "node9",
    ]
    .iter()
    .enumerate()
    .for_each(|(i, name)| {
        let node = DefaultNode::with_action(
            name.to_string(),
            NodeAction::new(i, root_id),
            &mut node_table,
        );
        let id = node.id();
        dag.add_node(node);
        dag.add_edge(id, vec![root_id]);
    });

    let mut env_var = EnvVar::new(node_table);
    env_var.set(ENV_DATA_SRC, "examples/ex3data1.mat");

    dag.set_env(env_var);
    dag.start().unwrap();

    let outputs = dag.get_outputs();
    let result = outputs.get(&root_id).unwrap().get_out().unwrap();
    let stdout = result.get::<Vec<u8>>().unwrap();
    log::info!("{}", String::from_utf8(stdout.clone()).unwrap());
    // let stderr = result.get::<Vec<u8>>().unwrap();
    // log::info!("{}", String::from_utf8(stderr.clone()).unwrap());

    let end = now.elapsed().as_secs();
    println!("time cost {:?}s", end); // 程序终止时间
}

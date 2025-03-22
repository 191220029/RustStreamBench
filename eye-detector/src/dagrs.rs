#[path = "common.rs"]
mod common;

use std::{env, fs::File, io::Write, sync::Arc, time::SystemTime};

use dagrs::{
    async_trait::async_trait, Action, Content, DefaultNode, EnvVar, Graph, InChannels, Node,
    NodeId, NodeTable, OutChannels, Output,
};

use {
    crossbeam_channel::{bounded, TryRecvError},
    opencv::{core, objdetect, prelude::*, types, videoio},
    std::collections::BTreeMap,
};

const IN_FILE: &str = "in_file";
const OUT_FILE: &str = "output_dagrs.avi";
const EYE: &str = "eye";
const FACE: &str = "face";
const DRAWER: &str = "drawer";
const WORKERS: &str = "workers";
const PRODUCERS: &str = "producers";
const READER: &str = "reader";
const WRITER: &str = "writer";

#[derive(Clone)]
struct StreamData {
    order: u64,
    frame: Mat,
    equalized: Option<Mat>,
    faces: Option<core::Vector<core::Rect>>,
}

pub struct Reorder {
    storage: BTreeMap<u64, StreamData>,
}

impl Reorder {
    fn new() -> Reorder {
        Reorder {
            storage: BTreeMap::<u64, StreamData>::new(),
        }
    }

    fn enqueue(&mut self, item: StreamData) {
        self.storage.insert(item.order, item);
    }

    fn remove(&mut self, order: u64) -> Option<StreamData> {
        if self.storage.contains_key(&order) {
            let removed_item = self.storage.remove(&order);
            match removed_item {
                Some(value) => return Some(value),
                None => {
                    panic!("Ordered removal failed")
                }
            }
        } else {
            return None;
        }
    }
}

struct Reader;
#[dagrs::async_trait::async_trait]
impl Action for Reader {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let workers: Vec<NodeId> = env.get(WORKERS).unwrap();
        let input_video: String = env.get(IN_FILE).unwrap();
        let mut video_in = videoio::VideoCapture::from_file(&input_video, videoio::CAP_FFMPEG).unwrap();
        let in_opened = videoio::VideoCapture::is_opened(&video_in).unwrap();
        if !in_opened {
            panic!("Unable to open input video {:?}!", input_video);
        }

        let mut index = 0;
        let mut order_id = 0;
        loop {
            let mut frame = Mat::default();
            video_in.read(&mut frame).unwrap();
            if frame.size().unwrap().width == 0 {
                break;
            }

            log::info!("Reader sending content to worker {:?}", workers[index]);
            out_channels
                .send_to(&workers[index], Content::new(StreamData {
                    order: order_id,
                    frame: frame,
                    equalized: None,
                    faces: None,
                }))
                .await
                .unwrap();
            index += 1;
            order_id += 1;
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

struct Face(usize);
#[dagrs::async_trait::async_trait]
impl Action for Face {
    async fn run(
        &self,
        in_channels: &mut InChannels,
        out_channels: &mut OutChannels,
        env: Arc<EnvVar>,
    ) -> Output {
        let reader_id: NodeId = env.get(READER).unwrap();
        let eye: NodeId = env.get(&format!("{}_{}", EYE, self.0)).unwrap();

        let face_xml =
        core::find_file("config/haarcascade_frontalface_alt.xml", true, false).unwrap();
        let mut face_detector = objdetect::CascadeClassifier::new(&face_xml).unwrap();
        
        while let Ok(content) = in_channels.recv_from(&reader_id).await {
            log::info!("Face({}) processing...", self.0);

            let content: &StreamData = content.get().unwrap();
            let mut content = content.clone();

            let equalized = common::prepare_frame(&content.frame).unwrap();

            // Detect faces
            let faces = common::detect_faces(&equalized, &mut face_detector).unwrap();

            // Out data
            content.equalized = Some(equalized);
            content.faces = Some(faces);
            out_channels.send_to(&eye, Content::new(content)).await.unwrap();
        }

        out_channels.close(&eye);

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
        let face: NodeId = env.get(&format!("{}_{}", FACE, self.0)).unwrap();
        let writer: NodeId = env.get(WRITER).unwrap();
        let eye_xml = core::find_file("config/haarcascade_eye.xml", true, false).unwrap();
        let mut eye_detector = objdetect::CascadeClassifier::new(&eye_xml).unwrap();

        while let Ok(content) = in_channels.recv_from(&face).await {
            log::info!("Eye({}) processing...", self.0);

            let content: &StreamData = content.get().unwrap();
            let mut content = content.clone();

            let equalized = match content.equalized {
                Some(ref x) => x,
                None => panic!("Empty value inside stream!"),
            };
            let faces = match content.faces {
                Some(ref x) => x,
                None => panic!("Empty value inside stream!"),
            };

            for face in faces {
                let eyes = common::detect_eyes(
                    &core::Mat::roi(equalized, face).unwrap().clone_pointee(),
                    &mut eye_detector,
                )
                .unwrap();

                common::draw_in_frame(&mut content.frame, &eyes, &face).unwrap();
            }
            out_channels.send_to(&writer, Content::new(content)).await.unwrap();
        }

        out_channels.close(&writer);
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
        let mut producers: Vec<NodeId> = env.get(PRODUCERS).unwrap();
        let mut reorder_engine = Reorder::new();
        let mut expected_ordered: u64 = 0;

        let mut index = 0;

        let input_video: String = env.get(IN_FILE).unwrap();
        let mut video_in = videoio::VideoCapture::from_file(&input_video, videoio::CAP_FFMPEG).unwrap();
        let in_opened = videoio::VideoCapture::is_opened(&video_in).unwrap();
        if !in_opened {
            panic!("Unable to open input video {:?}!", input_video);
        }
        let frame_size = core::Size::new(
            video_in
                .get(videoio::VideoCaptureProperties::CAP_PROP_FRAME_WIDTH as i32)
                .unwrap() as i32,
            video_in
                .get(videoio::VideoCaptureProperties::CAP_PROP_FRAME_HEIGHT as i32)
                .unwrap() as i32,
        );
        let fourcc = videoio::VideoWriter::fourcc('m', 'p', 'g', '1').unwrap();
        let fps_out = video_in
            .get(videoio::VideoCaptureProperties::CAP_PROP_FPS as i32)
            .unwrap();
        let mut video_out: videoio::VideoWriter =
            videoio::VideoWriter::new("output_dagrs.avi", fourcc, fps_out, frame_size, true)
                .unwrap();
        let out_opened = videoio::VideoWriter::is_opened(&video_out).unwrap();
        if !out_opened {
            panic!("Unable to open output video output_dagrs.avi!");
        }

        let mut collector = vec![];

        while producers.len() > 0 {
            if let Ok(content) = in_channels.recv_from(&producers[index]).await {
                log::info!("Writer received a packet");
                let content: &StreamData = content.get().unwrap();
                let mut content = content.clone();
                collector.push(content);
                index += 1;
            } else {
                producers.remove(index);
            }

            if index == producers.len() {
                index = 0;
            }
        }

        collector.sort_by_key(|content| content.order);
        // Write
        for mut content in collector {
            video_out.write(&mut content.frame).unwrap();
        }

        Output::empty()
    }
}

pub fn dagrs_eye_tracker(input_video: &String, nthreads: i32) -> opencv::Result<()> {
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let mut graph = Graph::new();
    let mut node_table = NodeTable::default();

    let mut node_ids: Vec<(String, NodeId)> = vec![];
    let mut workers = vec![];
    let mut producers = vec![];

    let reader =
        DefaultNode::with_action(format!("reader"), Reader, &mut node_table);
    let reader_id = reader.id();
    graph.add_node(reader);

    let writer = DefaultNode::with_action(format!("writer"), Writer, &mut node_table);
    let writer_id = writer.id();
    graph.add_node(writer);
    
    for i in 0..nthreads {
        let face_name = format!("{}_{}", FACE, i);
        let face =
            DefaultNode::with_action(face_name.clone(), Face(i as usize), &mut node_table);
        let face_id = face.id();
        graph.add_node(face);
        node_ids.push((face_name, face_id));
        workers.push(face_id);

        let eye_name = format!("{}_{}", EYE, i);
        let eye =
            DefaultNode::with_action(eye_name.clone(), Eye(i as usize), &mut node_table);
        let eye_id = eye.id();
        graph.add_node(eye);
        node_ids.push((eye_name, eye_id));
        producers.push(eye_id);
        graph.add_edge(face_id, vec![eye_id]);
        graph.add_edge(eye_id, vec![writer_id]);
    }

    graph.add_edge(reader_id, workers.clone());

    let mut env = EnvVar::new(node_table);
    env.set(IN_FILE, input_video.clone());
    env.set(READER, reader_id);
    env.set(WRITER, writer_id);
    env.set(WORKERS, workers);
    env.set(PRODUCERS, producers);
    for (k, v) in node_ids {
        env.set(&k, v);
    }

    graph.set_env(env);
    graph.start().unwrap();

    Ok(())
}

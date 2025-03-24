use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::test_group::TestGroup;

const FRAMES: [&str; 7] = [
    "sequential",
    "rust-ssp",
    "std-threads",
    "tokio",
    "rayon",
    "pipeliner",
    "dagrs",
];

const ITERATION_ARG: &str = "#!/bin/bash\n\niteration=\nwhile [[ $# -gt 0 ]]; do\n    case \"$1\" in\n\
                --iteration)\n            \
                iteration=\"$2\"\n        \
                    shift 2\n            \
                    ;;\n        \
                    *)\n        \
                        echo \"Unknown parameter: $1\"\n\
                                    exit 1\n\
                                                ;;\n\
                                                    esac\n\
                                                    done\n\
                                                    \n\
                if [[ -z \"$iteration\" ]]; then\n\
                    echo \"Error: --iteration argument is required.\"\n\
                        exit 1\n\
                fi\n\
            \n\
        if ! [[ \"$iteration\" =~ ^[0-9]+$ ]]; then\n\
        echo \"Error: --iteration must be a positive integer.\"\n\
        exit 1\n\
        fi\n";

fn bzip2(nthreads: usize) -> TestGroup {
    let location = "../bzip2";

    let mut scripts = vec![];
    for frame in FRAMES {
        let pth = format!("{}/test_{}.sh", location, frame);
        let f = File::create(&pth);
        let mut writer = BufWriter::new(f.unwrap());

        writer.write_all(ITERATION_ARG.as_bytes()).unwrap();

        writer
            .write_all(
                format!(
                    "rm -r workload/inputs/{0}\n\
            cp -r workload/backup workload/inputs/{0}\n\
            rm logs/{0}_*_iter${{iteration}}*.log\n",
                    frame
                )
                .as_bytes(),
            )
            .unwrap();

        writer.write_all(format!("./target/release/bzip2 {0} {1} compress workload/inputs/{0}/avi_video.avi > \"logs/{0}_compress_avi_video_iter${{iteration}}.log\" 2>&1\n\
./target/release/bzip2 {0} {1} compress workload/inputs/{0}/iso_file.iso > \"logs/{0}_compress_iso_file_iter${{iteration}}.log\" 2>&1 \n\
./target/release/bzip2 {0} {1} compress workload/inputs/{0}/wiki_data > \"logs/{0}_compress_iter${{iteration}}.log_wiki_data\" 2>&1 \n\
./target/release/bzip2 {0} {1} compress workload/inputs/{0}/jdk-17.0.12_linux-x64_bin.tar.gz > \"logs/{0}_compress_jdk_iter${{iteration}}.log\" 2>&1 \n\
\n\
./target/release/bzip2 {0} {1} decompress workload/inputs/{0}/avi_video.avi.bz2 > \"logs/{0}_decompress_avi_video_iter${{iteration}}.log\" 2>&1\n\
./target/release/bzip2 {0} {1} decompress workload/inputs/{0}/iso_file.iso.bz2 > \"logs/{0}_decompress_iso_file_iter${{iteration}}.log\" 2>&1 \n\
./target/release/bzip2 {0} {1} decompress workload/inputs/{0}/wiki_data.bz2 > \"logs/{0}_decompress_iter${{iteration}}.log_wiki_data\" 2>&1 \n\
./target/release/bzip2 {0} {1} decompress workload/inputs/{0}/jdk-17.0.12_linux-x64_bin.tar.gz.bz2 > \"logs/{0}_decompress_jdk_iter${{iteration}}.log\" 2>&1\n", frame, nthreads).as_bytes()).unwrap();

        scripts.push(PathBuf::from(pth));
    }
    TestGroup::new(PathBuf::from(location), scripts)
}

fn eye_detector(nthreads: usize) -> TestGroup {
    let location = "../eye-detector";

    let mut scripts = vec![];
    for frame in FRAMES {
        let pth = format!("{}/test_{}.sh", location, frame);
        let f = File::create(&pth);
        let mut writer = BufWriter::new(f.unwrap());

        writer.write_all(ITERATION_ARG.as_bytes()).unwrap();

        writer
            .write_all(
                format!(
                    "rm logs/{0}_*_iter${{iteration}}*.log\n\
            rm output_{0}.avi\n",
                    frame
                )
                .as_bytes(),
            )
            .unwrap();

        writer.write_all(format!("\
    ./target/release/eye-detector {0} {1} ./inputs/mixed_15s.mp4 > logs/{0}_mixed_iter${{iteration}}.log 2>&1\n\
    ./target/release/eye-detector {0} {1} ./inputs/one_face_15s.mp4 > logs/{0}_one_face_iter${{iteration}}.log 2>&1\n\
    ./target/release/eye-detector {0} {1} ./inputs/several_faces_15s.mp4 > logs/{0}_several_faces_iter${{iteration}}.log 2>&1\n", frame, nthreads).as_bytes()).unwrap();

        scripts.push(PathBuf::from(pth));
    }
    TestGroup::new(PathBuf::from(location), scripts)
}

fn image_processing(nthreads: usize) -> TestGroup {
    let location = "../image-processing";

    let mut scripts = vec![];
    for frame in FRAMES {
        let pth = format!("{}/test_{}.sh", location, frame);
        let f = File::create(&pth);
        let mut writer = BufWriter::new(f.unwrap());

        writer.write_all(ITERATION_ARG.as_bytes()).unwrap();

        writer
            .write_all(format!("rm logs/{0}_*_iter${{iteration}}*.log\n", frame).as_bytes())
            .unwrap();

        writer.write_all(format!("\
    ./target/release/image-processing {0} {1} input_big > logs/{0}_big_iter${{iteration}}.log 2>&1\n\
    ./target/release/image-processing {0} {1} input_mixed > logs/{0}_mixed_iter${{iteration}}.log 2>&1\n\
    ./target/release/image-processing {0} {1} input_small > logs/{0}_small_iter${{iteration}}.log 2>&1\n", frame, nthreads).as_bytes()).unwrap();

        scripts.push(PathBuf::from(pth));
    }
    TestGroup::new(PathBuf::from(location), scripts)
}

fn micro_bench(nthreads: usize) -> TestGroup {
    let location = "../micro-bench";

    let mut scripts = vec![];
    for frame in FRAMES {
        let pth = format!("{}/test_{}.sh", location, frame);
        let f = File::create(&pth);
        let mut writer = BufWriter::new(f.unwrap());

        writer.write_all(ITERATION_ARG.as_bytes()).unwrap();

        writer
            .write_all(format!("rm logs/{0}_*_iter${{iteration}}*.log\n", frame).as_bytes())
            .unwrap();

        writer.write_all(format!("./target/release/micro-bench {0} 2048 {1} 3000 2000 > logs/{0}_iter${{iteration}}.log 2>&1\n", frame, nthreads).as_bytes()).unwrap();

        scripts.push(PathBuf::from(pth));
    }
    TestGroup::new(PathBuf::from(location), scripts)
}

pub fn generate_test_script(nthreads: usize) -> Vec<TestGroup> {
    let mut res = vec![];

    log::info!("Generating scripts for bzip2...");
    res.push(bzip2(nthreads));
    log::info!("Generating scripts for eye-detector...");
    res.push(eye_detector(nthreads));
    log::info!("Generating scripts for image-processing...");
    res.push(image_processing(nthreads));
    log::info!("Generating scripts for micro-bench...");
    res.push(micro_bench(nthreads));

    res
}

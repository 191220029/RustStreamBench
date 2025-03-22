use std::{
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    println!("Hello, world!");

    let location = vec![
        "/home/xiaolongfu/RustStreamBench/eye-detector",
        "/home/xiaolongfu/RustStreamBench/image-processing",
        "/home/xiaolongfu/RustStreamBench/micro-bench",
    ];

    let frames = vec![
        "sequential",
        "rust-ssp",
        "std-threads",
        "tokio",
        "rayon",
        "pipeliner",
        "dagrs",
    ];

    let location = "/home/xiaolongfu/RustStreamBench/eye-detector";

    for frame in frames {
        let f = File::create(format!("{}/test_{}.sh", location, frame));
        let mut writer = BufWriter::new(f.unwrap());

        writer.write_all("#!/bin/bash\n\n# 解析命令行参数\niteration=\nwhile [[ $# -gt 0 ]]; do\n    case \"$1\" in\n\
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
                                                    # 验证参数\n\
                if [[ -z \"$iteration\" ]]; then\n\
                    echo \"Error: --iteration argument is required.\"\n\
                        exit 1\n\
                fi\n\
            \n\
        if ! [[ \"$iteration\" =~ ^[0-9]+$ ]]; then\n\
        echo \"Error: --iteration must be a positive integer.\"\n\
        exit 1\n\
        fi\n".as_bytes()).unwrap();

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
./target/release/eye-detector {0} 10 ./inputs/mixed_15s.mp4 > logs/{0}_mixed_iter${{iteration}}.log 2>&1\n\
./target/release/eye-detector {0} 10 ./inputs/one_face_15s.mp4 > logs/{0}_one_face_iter${{iteration}}.log 2>&1\n\
./target/release/eye-detector {0} 10 ./inputs/several_faces_15s.mp4 > logs/{0}_several_faces_iter${{iteration}}.log 2>&1\n", frame).as_bytes()).unwrap();
    }
}

use csv::Writer;
use heck::ToTitleCase;
use regex::Regex;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct LogData {
    framework: String,
    workload: String,
    iteration: u32,
    time: String,
}

pub fn collect_logs(log_dir: &str, output_csv: &str) -> Result<(), Box<dyn Error>> {
    // 编译正则表达式
    let re = Regex::new(
        r"^(?P<framework>[a-zA-Z-]+)_(?P<operation_workload>[\w._-]*)iter(?P<iteration>\d+)",
    )?;

    let mut rows = Vec::new();

    // 遍历日志目录
    for entry in fs::read_dir(log_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("log") {
            continue;
        }

        // 解析文件名
        let filename = path.file_name().unwrap().to_string_lossy();
        let caps = match re.captures(&filename) {
            Some(c) => c,
            None => continue,
        };

        // 提取字段
        let framework = caps["framework"].to_string();
        let operation_workload = caps["operation_workload"].replace(".log_", "_");
        let iteration = caps["iteration"].parse::<u32>()?;

        // 读取执行时间
        let file = File::open(&path)?;
        let lines: Vec<_> = BufReader::new(file).lines().collect::<Result<_, _>>()?;
        let last_line = match lines.last() {
            Some(line) => line.trim(),
            None => continue,
        };

        if !last_line.contains("Execution time:") {
            continue;
        }

        let time_str = match last_line.split_whitespace().nth_back(1) {
            Some(t) => t,
            None => continue,
        };

        // 格式化数据
        let workload = operation_workload.replace('_', " ").to_title_case();

        rows.push(LogData {
            framework,
            workload,
            iteration,
            time: format!("{:.6}", time_str.parse::<f64>()?),
        });
    }

    // 按框架、负载、迭代排序
    rows.sort_by(|a, b| {
        a.framework
            .cmp(&b.framework)
            .then(a.workload.cmp(&b.workload))
            .then(a.iteration.cmp(&b.iteration))
    });

    // 写入CSV文件
    let mut wtr = Writer::from_path(output_csv)?;
    wtr.write_record(&["Framework", "Workload", "Iteration", "ExecutionTime(s)"])?;

    for row in rows {
        wtr.write_record(&[
            row.framework,
            row.workload,
            row.iteration.to_string(),
            row.time,
        ])?;
    }

    wtr.flush()?;
    println!("saving data to {}", output_csv);

    Ok(())
}

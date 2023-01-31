use std::{
    fs::{File, OpenOptions},
    process::Stdio,
    thread::sleep_ms,
    time::Duration,
};

use clap::Parser;
use cmd_lib::run_cmd;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Whether test passed or failed (either 'p' or 'f')
    #[arg(short, default_value_t = 'p')]
    result_type: char,
    /// Message to log
    #[arg(short, default_value = "")]
    message: String,
    /// Optional explanation
    #[arg(short, default_value = "")]
    explanation: String,
    /// Whether to output the log to console or not
    #[arg(short)]
    output: bool,
}

fn main() {
    let args = Args::parse();

    let mut failed = false;
    let dir = "results.csv";
    let file_exists = std::path::Path::new(&dir).exists();
    let results_file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
        .open(dir)
        .unwrap();

    if args.output {
        let total_records = csv::Reader::from_reader(results_file).records().count();

        let step_summary = "$GITHUB_STEP_SUMMARY";

        // File consumed counting, so re-open
        let results_file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .create(true)
            .open(dir)
            .unwrap();
        let mut reader = csv::Reader::from_reader(&results_file);
        for (idx, record) in reader.records().enumerate() {
            let record = record.unwrap();

            let mut fields = record.iter();
            let result_type = fields.next().unwrap();
            let message = fields.next().unwrap();
            let explanation = fields.next().unwrap();

            let idx = idx + 1;
            let mut message = format!("[{idx}/{total_records}] {result_type} | {message}");

            if result_type == "PASS" {
                message = ":x:".to_owned() + &message;
            } else {
                message = ":white_check_mark:".to_owned() + &message;
                failed = true;
            }

            if !explanation.is_empty() {
                let explanation = "> ".to_owned() + &explanation;
                message.push_str(&explanation);
            }

            std::process::Command::new("echo")
                .arg(message.clone())
                .spawn()
                .unwrap();

            run_cmd!(bash -c "echo '$message' >> $step_summary").unwrap();

            std::thread::sleep(Duration::from_millis(100));
        }

        if failed {
            File::create("failed").unwrap();
        }
    } else {
        let mut writer = csv::Writer::from_writer(results_file);

        if !file_exists {
            writer
                .write_record(&["Result", "Message", "Explanation"])
                .unwrap();
        }

        let result = if args.result_type == 'p' {
            "PASS"
        } else {
            "FAIL"
        };

        writer
            .write_record(&[result, &args.message, &args.explanation])
            .unwrap();
    }
}

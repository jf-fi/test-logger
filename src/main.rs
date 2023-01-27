use clap::Parser;
use colored::Colorize;

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

    if let Some(workspace) = option_env!("GITHUB_WORKSPACE") {
        let results_file = workspace.to_owned() + "/test/results.csv";
        if args.output {
            let mut reader = csv::Reader::from_path(results_file).unwrap();
            let total_records = reader.records().count();
            for (idx, record) in reader.records().enumerate() {
                let record = record.unwrap();

                let mut fields = record.iter();
                let result_type = fields.next().unwrap();
                let message = fields.next().unwrap();
                let explanation = fields.next().unwrap();

                let mut message = format!("[{idx}/{total_records}] {result_type} | {message}");
                if !explanation.is_empty() {
                    message.push_str("\n    ");
                    message += explanation;
                }

                if result_type == "PASS" {
                    println!("{}", message.green());
                } else {
                    println!("{}", message.red());
                }
            }
        } else {
            let mut writer = csv::Writer::from_path(results_file).unwrap();
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
}

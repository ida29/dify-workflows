use clap::Parser;
use dify_linter::{print_json, print_report, DifyDsl, DifyLinter};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

/// Dify DSL Linter / Validator
#[derive(Parser, Debug)]
#[command(name = "dify-linter")]
#[command(about = "Validate Dify DSL YAML files before import")]
#[command(version)]
struct Args {
    /// YAML file to lint
    file: PathBuf,

    /// Output as JSON
    #[arg(long)]
    json: bool,

    /// Auto-fix issues (coming soon)
    #[arg(long)]
    fix: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    // Read file
    let content = match fs::read_to_string(&args.file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Parse YAML
    let dsl: DifyDsl = match serde_yaml::from_str(&content) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error parsing YAML: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Run linter
    let linter = DifyLinter::new(dsl);
    let result = linter.lint();

    // Output
    if args.json {
        print_json(&result);
    } else {
        print_report(&result);
    }

    // Exit code
    if result.valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

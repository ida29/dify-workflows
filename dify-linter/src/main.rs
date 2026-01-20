use clap::Parser;
use dify_linter::{print_json, print_report, DifyDsl, DifyLinter};
use serde_json::Value;
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

    // Detect and extract YAML content
    let yaml_content = if content.trim_start().starts_with('{') {
        // JSON format - try to extract 'data' field which contains YAML
        match serde_json::from_str::<Value>(&content) {
            Ok(json) => {
                if let Some(data) = json.get("data").and_then(|d| d.as_str()) {
                    data.to_string()
                } else {
                    eprintln!("Error: JSON file does not contain 'data' field with YAML");
                    return ExitCode::FAILURE;
                }
            }
            Err(e) => {
                eprintln!("Error parsing JSON wrapper: {}", e);
                return ExitCode::FAILURE;
            }
        }
    } else {
        content
    };

    // Parse YAML
    let dsl: DifyDsl = match serde_yaml::from_str(&yaml_content) {
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

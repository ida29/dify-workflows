use crate::types::{LintError, LintResult, Severity};
use colored::*;

/// Print lint report to stdout
pub fn print_report(result: &LintResult) {
    println!("{}", "=".repeat(60));
    println!("DIFY DSL LINT REPORT");
    println!("{}", "=".repeat(60));

    if !result.errors.is_empty() {
        println!(
            "\n{} ({}):\n",
            "ERRORS".red().bold(),
            result.errors.len()
        );
        for err in &result.errors {
            print_lint_error(err);
        }
    }

    if !result.warnings.is_empty() {
        println!(
            "\n{} ({}):\n",
            "WARNINGS".yellow().bold(),
            result.warnings.len()
        );
        for warn in &result.warnings {
            print_lint_error(warn);
        }
    }

    if result.errors.is_empty() && result.warnings.is_empty() {
        println!("\n{}\n", "No issues found!".green().bold());
    }

    println!("{}", "=".repeat(60));
    println!(
        "Summary: {} errors, {} warnings",
        result.errors.len(),
        result.warnings.len()
    );
    println!("{}", "=".repeat(60));
}

fn print_lint_error(err: &LintError) {
    let prefix = match err.severity {
        Severity::Error => "x".red(),
        Severity::Warning => "!".yellow(),
    };

    println!("  {} [{}] {}", prefix, err.node_id, err.node_title);
    println!("    -> {}", err.message);
    if let Some(hint) = &err.fix_hint {
        println!("    {} Fix: {}", "=>".cyan(), hint);
    }
    println!();
}

/// Print result as JSON
pub fn print_json(result: &LintResult) {
    match serde_json::to_string_pretty(result) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing result: {}", e),
    }
}

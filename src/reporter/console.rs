use colored::*;

use crate::domain::types::{Detected, FileReport};
use crate::error::MlCheckError;

/// Print check results to the console. I/O boundary.
pub fn report_to_console(report: &FileReport) -> Result<(), MlCheckError> {
    let header = r#"
     _ __ ___ | | ___| |__   ___  ___| | __
    | '_ ` _ \| |/ __| '_ \ / _ \/ __| |/ /
    | | | | | | | (__| | | |  __/ (__|   <
    |_| |_| |_|_|\___|_| |_|\___|\___|_|\_\
"#;
    println!("{}", header.bold().truecolor(0, 165, 255));
    println!(
        "File: {}",
        report
            .file_info
            .path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".into())
    );
    println!();

    for result in &report.results {
        let status = match result.detected {
            Detected::Present => "present".bold().truecolor(0, 165, 255).to_string(),
            Detected::Absent => "absent".bold().truecolor(255, 165, 0).to_string(),
        };
        println!("{}: {}", result.description, status);
    }

    println!();
    println!(
        "Score: {:.0}%",
        report.score.percentage()
    );

    Ok(())
}

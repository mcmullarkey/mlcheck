use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::domain::types::{Detected, FileReport};
use crate::error::MlCheckError;

/// Write check results to a CSV file. I/O boundary.
pub fn report_to_csv(report: &FileReport, output_dir: &Path) -> Result<(), MlCheckError> {
    let csv_path = output_dir.join("mlcheck_output.csv");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&csv_path)
        .map_err(|e| MlCheckError::FileReadError {
            path: csv_path.clone(),
            source: e,
        })?;

    // Write header if file is empty
    let metadata = file.metadata().map_err(|e| MlCheckError::FileReadError {
        path: csv_path.clone(),
        source: e,
    })?;

    if metadata.len() == 0 {
        writeln!(file, "file_name,target,description,detected,group_id,datetime")
            .map_err(|e| MlCheckError::FileReadError {
                path: csv_path.clone(),
                source: e,
            })?;
    }

    let file_name = report
        .file_info
        .path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".into());

    for result in &report.results {
        let detected = match result.detected {
            Detected::Present => "true",
            Detected::Absent => "false",
        };
        writeln!(
            file,
            "{},{},{},{},{},{}",
            file_name,
            result.rule_id.0,
            result.description,
            detected,
            report.group_id,
            report.timestamp,
        )
        .map_err(|e| MlCheckError::FileReadError {
            path: csv_path.clone(),
            source: e,
        })?;
    }

    Ok(())
}

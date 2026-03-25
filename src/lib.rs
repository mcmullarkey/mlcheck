pub mod cli;
pub mod domain;
pub mod error;
pub mod reporter;
pub mod rules;
pub mod scanner;

use std::path::Path;

use chrono::Utc;
use uuid::Uuid;

use crate::cli::Config;
use crate::domain::checks::score_from_results;
use crate::domain::types::{
    FileInfo, FileReport, OutputFormat, ScanTarget, SourceContent, SourceFileType,
};
use crate::error::MlCheckError;
use crate::reporter::console::report_to_console;
use crate::reporter::csv::report_to_csv;
use crate::reporter::sqlite::report_to_sqlite;
use crate::rules::pattern::evaluate_all;
use crate::rules::registry::rules_for;
use crate::scanner::file_reader::read_file;

/// Resolve the user-level data directory for mlcheck output files.
/// Uses the platform-standard data dir (e.g. ~/Library/Application Support/mlcheck on macOS,
/// ~/.local/share/mlcheck on Linux). Creates it if it doesn't exist.
fn output_dir(output_format: OutputFormat) -> Result<std::path::PathBuf, MlCheckError> {
    match output_format {
        OutputFormat::Console => Ok(std::path::PathBuf::new()), // unused, but satisfies the type
        OutputFormat::Csv | OutputFormat::Sqlite => {
            let base = dirs::data_dir().ok_or(MlCheckError::NoDataDirectory)?;
            let dir = base.join("mlcheck");
            std::fs::create_dir_all(&dir).map_err(|e| MlCheckError::FileReadError {
                path: dir.clone(),
                source: e,
            })?;
            Ok(dir)
        }
    }
}

/// Main orchestrator. Wires I/O shell to pure core.
pub fn run(config: Config) -> Result<(), MlCheckError> {
    let out_dir = match config.output_dir {
        Some(dir) => {
            std::fs::create_dir_all(&dir).map_err(|e| MlCheckError::FileReadError {
                path: dir.clone(),
                source: e,
            })?;
            dir
        }
        None => output_dir(config.output_format)?,
    };
    match &config.target {
        ScanTarget::SingleFile(path) => {
            check_file(path, config.output_format, &out_dir)?;
        }
        ScanTarget::Directory(dir) => {
            check_directory(dir, config.output_format, &out_dir)?;
        }
    }
    Ok(())
}

fn check_file(path: &Path, output_format: OutputFormat, out_dir: &Path) -> Result<(), MlCheckError> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| {
            MlCheckError::UnsupportedExtension("no extension".into())
        })?;

    let file_type = SourceFileType::from_extension(ext).ok_or_else(|| {
        MlCheckError::UnsupportedExtension(ext.to_string())
    })?;

    let language = file_type.language();
    let framework = file_type.default_framework();

    // I/O: read file
    let raw = read_file(path)?;

    // For notebooks, extract only code cells before parsing
    let source = if file_type == SourceFileType::JupyterNotebook {
        scanner::notebook::extract_code_cells(&raw, path)?
    } else {
        raw
    };

    // Pure core
    let content = SourceContent::parse(&source, language);
    let rules = rules_for(language, framework);
    let results = evaluate_all(&rules, &content);
    let score = score_from_results(&results);

    // I/O: generate metadata and report
    let report = FileReport {
        file_info: FileInfo {
            path: path.to_path_buf(),
            language,
            framework,
        },
        results,
        score,
        group_id: Uuid::new_v4().to_string(),
        timestamp: Utc::now().to_rfc3339(),
    };

    // Always show console output
    report_to_console(&report)?;

    // Write to file/DB if requested
    match output_format {
        OutputFormat::Console => {}
        OutputFormat::Csv => {
            report_to_csv(&report, out_dir)?;
            println!("Results written to {}", out_dir.join("mlcheck_output.csv").display());
        }
        OutputFormat::Sqlite => {
            report_to_sqlite(&report, out_dir)?;
            println!("Results written to {}", out_dir.join("mlcheck_output.db").display());
        }
    }

    Ok(())
}

fn check_directory(dir: &Path, output_format: OutputFormat, out_dir: &Path) -> Result<(), MlCheckError> {
    let entries = std::fs::read_dir(dir).map_err(|e| MlCheckError::FileReadError {
        path: dir.to_path_buf(),
        source: e,
    })?;

    let mut found = false;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if SourceFileType::from_extension(ext).is_some() {
                    check_file(&path, output_format, out_dir)?;
                    found = true;
                }
            }
        }
    }

    if !found {
        return Err(MlCheckError::EmptyDirectory(dir.to_path_buf()));
    }

    Ok(())
}

use anyhow::{Context, Result};
use chrono::Utc;
use clap::Parser;
use colored::*;
use log::{info, error};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up");

    // Define checks
    let checks = vec![
        Check {
            target: String::from("train_test_split"),
            description: String::from("Splits data into train and test"),
        },
        Check {
            target: String::from("stratify"),
            description: String::from("Ensures class balance in train and test datasets"),
        },
        Check {
            target: String::from("random_state"),
            description: String::from("Sets seed for reproducible train and test datasets")
        }
    ];

    // Parse command-line arguments
    let args = Cli::parse();

    validate_arguments(&args)?;

    let content = read_file_content(&args.path)?;

    let output_path = Path::new(&args.output);

    let mut output_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&output_path)
        .with_context(|| format!("Could not open file `{}`", output_path.display()))?;

    // Write CSV header if file is empty
    if output_file.metadata()?.len() == 0 {
        write_csv_header(&mut output_file)?;
    }

    // Perform checks
    for check in &checks {
        let pattern_found = content.lines().any(|line| line.contains(&check.target));

        write_check_result(&mut output_file, &args.path, &check, pattern_found)?;

        display_check_result(&check, pattern_found);
    }

    info!("Results appended to {}", output_path.display());

    Ok(())
}

/// Struct representing a check.
#[derive(Debug)]
struct Check {
    target: String,
    description: String,
}

/// Validate command-line arguments.
fn validate_arguments(args: &Cli) -> Result<()> {
    if !args.path.exists() {
        return Err(anyhow::anyhow!("Input file does not exist"));
    }
    Ok(())
}

/// Read content from the specified file.
fn read_file_content(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("Could not read file `{}`", path.display()))
}

/// Write CSV header if file is empty.
fn write_csv_header(output_file: &mut fs::File) -> Result<()> {
    writeln!(
        output_file,
        "file_name,target,description,detected,datetime"
    )
    .with_context(|| "Failed to write header to CSV file")
}

/// Write check result to the CSV file.
fn write_check_result(output_file: &mut fs::File, path: &PathBuf, check: &Check, pattern_found: bool) -> Result<()> {
    writeln!(
        output_file,
        "{},{},{},{},{}",
        path.file_name().ok_or_else(|| anyhow::anyhow!("Could not get file name"))?.to_string_lossy(),
        check.target,
        check.description,
        pattern_found,
        Utc::now()
    )
    .with_context(|| "Failed to write check result to CSV file")
}

/// Display the result of a check.
fn display_check_result(check: &Check, pattern_found: bool) {
    let status = if pattern_found {
        format!("present").bold().truecolor(0, 165, 255).to_string()
    } else {
        format!("absent").bold().truecolor(255, 165, 0).to_string()
    };
    println!("{}: {}", check.description, status);
}

/// Command-line arguments structure.
#[derive(Debug, Parser)]
#[clap(name = "pattern_search")]
struct Cli {
    /// The path to the file to read
    #[clap(short, long)]
    path: PathBuf,
    /// The name of the output CSV file
    #[clap(short, long, default_value = "output.csv")]
    output: String,
    /// Adding verbosity flag for debugging
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}





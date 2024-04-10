use anyhow::{Context, Result};
use chrono::Utc;
use clap::Parser;
use colored::*;
use log::info;
use std::fs::{OpenOptions};
use std::io::{Write};
use std::path::{Path};

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let pattern_found = content.lines().any(|line| line.contains(&args.target));

    let output_path = Path::new(&args.output);

    if !output_path.exists() {
        create_csv_with_header(&output_path)?;
    }

    let mut output_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&output_path)
        .with_context(|| format!("could not open file `{}`", output_path.display()))?;

    writeln!(
        output_file,
        "{},{},{},{}",
        args.path.file_name().unwrap().to_string_lossy(),
        args.target,
        pattern_found,
        Utc::now()
    )
    .with_context(|| format!("failed to write data to CSV file"))?;

    if pattern_found {
        println!(
            "{}",
            "Machine Learning Checklist:\nSplits data into train and test: ".to_owned() + &"pass".bold().truecolor(0, 165, 255).to_string()
        );
    } else {
        println!(
            "{}",
            "Machine Learning Checklist:\nSplits data into train and test: ".to_owned() + &"fail".bold().truecolor(255, 165, 0).to_string()
        );
    }

    info!("results appended to {}", output_path.display());

    Ok(())
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
#[clap(name = "pattern_search")]
struct Cli {
    /// The target pattern to look for
    #[clap(short, long, default_value = "train_test_split")]
    target: String,
    /// The path to the file to read
    #[clap(short, long)]
    path: std::path::PathBuf,
    /// The name of the output CSV file
    #[clap(short, long, default_value = "output.csv")]
    output: String,
    /// Adding verbosity flag for debugging
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn create_csv_with_header(path: &Path) -> Result<()> {
    let mut output_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .with_context(|| format!("could not create file `{}`", path.display()))?;

    writeln!(
        output_file,
        "file_name,target,detected,datetime"
    )
    .with_context(|| format!("failed to write header to CSV file"))?;

    Ok(())
}

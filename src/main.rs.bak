use anyhow::{Context, Result};
use chrono::Utc;
use clap::Parser;
use colored::*;
use log::{info};
use uuid::Uuid;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::fs::{self, OpenOptions};
use std::io::{Write};
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up");

    // Define Python and R checks
    let py_checks = vec![
        Check {
            target: String::from("sklearn"),
            description: String::from("Imports the scikit-learn library"),
        },
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
        },
        Check {
            target: String::from("Pipeline"),
            description: String::from("Uses a pipeline to guard against data leakage")
        }
    ];

    let r_checks = vec![
        Check {
            target: String::from("tidymodels"),
            description: String::from("Imports the tidymodels library"),
        },
        Check {
            target: String::from("initial_split"),
            description: String::from("Splits data into train and test"),
        },
        Check {
            target: String::from("strata"),
            description: String::from("Ensures class balance in train and test datasets"),
        },
        Check {
            target: String::from("set.seed"),
            description: String::from("Sets seed for reproducible train and test datasets")
        },
        Check {
            target: String::from("recipe"),
            description: String::from("Uses a recipe to guard against data leakage")
        }
    ];

    // Parse command-line arguments
    let args = Cli::parse();

    validate_arguments(&args)?;

    if args.path.is_dir() {
        handle_folder(&args.path, &args.output, &py_checks, &r_checks)?;
    } else {
        handle_file(&args.path, &args.output, &py_checks, &r_checks)?;
    }

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

fn handle_folder(folder_path: &PathBuf, output_format: &str, py_checks: &[Check], r_checks: &[Check]) -> Result<()> {
    let files_to_check = fs::read_dir(folder_path)
        .with_context(|| format!("Failed to read directory `{}`", folder_path.display()))?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.path().is_file() {
                    Some(e.path())
                } else {
                    None
                }
            })
        });

    for file_path in files_to_check {
        handle_file(&file_path, output_format, py_checks, r_checks)?;
    }

    Ok(())
}

fn handle_file(file_path: &PathBuf, output_format: &str, py_checks: &[Check], r_checks: &[Check]) -> Result<()> {
    let content = read_file_content(file_path)?;

    display_mlcheck_header(file_path);

    let extension = file_path.extension().and_then(|ext| ext.to_str());

    let checks = match extension {
        Some("py") | Some("ipynb") => py_checks,
        Some("R") | Some("Rmd") => r_checks,
        _ => return Err(anyhow::anyhow!("Unsupported file extension")),
    };

    match output_format {
        "csv" => {
            let output_path = Path::new("mlcheck_output.csv");
            let mut output_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&output_path)
                .with_context(|| format!("Could not open file `{}`", output_path.display()))?;

            // Write CSV header if file is empty
            if output_file.metadata()?.len() == 0 {
                write_csv_header(&mut output_file)?;
            }

            // Generate unique group identifier for this file
            let group_id = generate_group_id();

            // Perform checks
            for check in checks {
                let pattern_found = content.lines().any(|line| line.contains(&check.target));

                write_check_result(&mut output_file, file_path, check, pattern_found, &group_id)?;

                display_check_result(check, pattern_found);
            }

            info!("Results appended to {}", output_path.display());
        },
        "sql" => {
            let conn = Connection::open("mlcheck_output.db").with_context(|| "Failed to open SQLite database")?;
            create_table(&conn)?;

            // Generate unique group identifier for this file
            let group_id = generate_group_id();

            // Perform checks
            for check in checks {
                let pattern_found = content.lines().any(|line| line.contains(&check.target));

                insert_check_result(&conn, file_path, check, pattern_found, &group_id)?;

                display_check_result(check, pattern_found);
            }

            info!("Results saved to SQLite database mlcheck_output.db");
        },
        _ => {
            println!("Invalid output format specified. Please use 'csv' or 'sql'.");
        }
    }

    // Display percentage of checks that are present
    let present_checks_percentage = calculate_present_checks_percentage(checks, &content);
    println!("Percentage of checks marked 'present': {:.0}%", present_checks_percentage);

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
        "file_name,target,description,detected,group_id,datetime"
    )
    .with_context(|| "Failed to write header to CSV file")
}

/// Write check result to the CSV file with group identifier.
fn write_check_result(output_file: &mut fs::File, path: &PathBuf, check: &Check, pattern_found: bool, group_id: &Uuid) -> Result<()> {
    writeln!(
        output_file,
        "{},{},{},{},{},{}",
        path.file_name().ok_or_else(|| anyhow::anyhow!("Could not get file name"))?.to_string_lossy(),
        check.target,
        check.description,
        pattern_found,
        group_id,
        Utc::now()
    )
    .with_context(|| "Failed to write check result to CSV file")
}

fn display_mlcheck_header(path: &PathBuf) {
    let mlcheck = r#"                         
     _ __ ___ | | ___| |__   ___  ___| | __
    | '_ ` _ \| |/ __| '_ \ / _ \/ __| |/ /
    | | | | | | | (__| | | |  __/ (__|   < 
    |_| |_| |_|_|\___|_| |_|\___|\___|_|\_\
 "#;
    println!("{}", mlcheck.bold().truecolor(0, 165, 255).to_string());
    println!("For the file: {}", path.file_name().expect("Could not get file name").to_string_lossy());
}

/// Display the result of a check.
fn display_check_result(check: &Check, pattern_found: bool) {
    let status = if pattern_found {
        "present".bold().truecolor(0, 165, 255).to_string()
    } else {
        "absent".bold().truecolor(255, 165, 0).to_string()
    };

    println!("{}: {}", check.description, status);
}

// Calculate % of checks that are present
fn calculate_present_checks_percentage(checks: &[Check], content: &str) -> f64 {
    let present_checks_count = checks.iter().filter(|check| {
        content.lines().any(|line| line.contains(&check.target))
    }).count();
    let total_checks_count = checks.len();
    present_checks_count as f64 / total_checks_count as f64 * 100.0
}

/// Generate a unique group identifier for each file.
fn generate_group_id() -> Uuid {
    Uuid::new_v4()
}

fn create_table(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mlcheck_results (
             id INTEGER PRIMARY KEY,
             file_name TEXT,
             target TEXT,
             description TEXT,
             detected BOOLEAN,
             group_id TEXT,
             datetime TEXT
         )",
        [],
    )?;
    Ok(())
}

fn insert_check_result(conn: &Connection, path: &PathBuf, check: &Check, pattern_found: bool, group_id: &Uuid) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO mlcheck_results (file_name, target, description, detected, group_id, datetime)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            path.file_name().ok_or_else(|| rusqlite::Error::InvalidParameterName("Could not get file name".to_string()))?.to_string_lossy(),
            check.target,
            check.description,
            pattern_found,
            group_id.to_string(),
            Utc::now().to_rfc3339()
        ],
    )?;
    Ok(())
}

/// Command-line arguments structure.
#[derive(Debug, Parser)]
#[clap(name = "pattern_search")]
struct Cli {
    /// The path to the file to read
    #[clap(short, long)]
    path: PathBuf,
    /// The name of the output CSV file
    #[clap(short, long, default_value = "sql")]
    output: String,
    /// Adding verbosity flag for debugging
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}






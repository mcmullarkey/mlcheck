use std::path::PathBuf;

use clap::Parser;

use crate::domain::types::{OutputFormat, ScanTarget};

/// mlcheck — Check ML code files for best practices.
#[derive(Debug, Parser)]
#[clap(name = "mlcheck", version)]
pub struct CliArgs {
    /// Path to a file or directory to check
    #[clap(short, long)]
    pub path: PathBuf,

    /// Output format: console, csv, or sql
    #[clap(short, long, default_value = "console")]
    pub output: String,

    /// Directory to write output files (csv/sql). Defaults to platform data dir.
    #[clap(long)]
    pub output_dir: Option<PathBuf>,
}

/// Validated configuration derived from CLI args.
#[derive(Debug, Clone)]
pub struct Config {
    pub target: ScanTarget,
    pub output_format: OutputFormat,
    pub output_dir: Option<PathBuf>,
}

impl Config {
    pub fn from_args(args: CliArgs) -> Result<Self, String> {
        if !args.path.exists() {
            return Err(format!("Path does not exist: {}", args.path.display()));
        }

        let target = if args.path.is_dir() {
            ScanTarget::Directory(args.path)
        } else {
            ScanTarget::SingleFile(args.path)
        };

        let output_format = match args.output.as_str() {
            "console" => OutputFormat::Console,
            "csv" => OutputFormat::Csv,
            "sql" => OutputFormat::Sqlite,
            other => return Err(format!("Unknown output format: {other}. Use console, csv, or sql.")),
        };

        Ok(Config {
            target,
            output_format,
            output_dir: args.output_dir,
        })
    }
}

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MlCheckError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Unsupported file extension: {0}")]
    UnsupportedExtension(String),

    #[error("Failed to read file {path}: {source}")]
    FileReadError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to parse notebook {path}: {source}")]
    NotebookParseError {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error("SQLite error: {0}")]
    SqliteError(#[from] rusqlite::Error),

    #[error("Could not determine user data directory")]
    NoDataDirectory,

    #[error("No supported files found in directory: {0}")]
    EmptyDirectory(PathBuf),
}

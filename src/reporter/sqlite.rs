use std::path::Path;

use rusqlite::{params, Connection};

use crate::domain::types::{Detected, FileReport};
use crate::error::MlCheckError;

/// Write check results to a SQLite database. I/O boundary.
pub fn report_to_sqlite(report: &FileReport, output_dir: &Path) -> Result<(), MlCheckError> {
    let db_path = output_dir.join("mlcheck_output.db");

    let conn = Connection::open(&db_path).map_err(MlCheckError::SqliteError)?;

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
    )
    .map_err(MlCheckError::SqliteError)?;

    let file_name = report
        .file_info
        .path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".into());

    for result in &report.results {
        let detected = match result.detected {
            Detected::Present => true,
            Detected::Absent => false,
        };
        conn.execute(
            "INSERT INTO mlcheck_results (file_name, target, description, detected, group_id, datetime)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                file_name,
                result.rule_id.0,
                result.description,
                detected,
                report.group_id,
                report.timestamp,
            ],
        )
        .map_err(MlCheckError::SqliteError)?;
    }

    Ok(())
}

use std::fs;
use std::path::Path;

use crate::error::MlCheckError;

/// Read a file's contents as a string. This is an I/O boundary function.
pub fn read_file(path: &Path) -> Result<String, MlCheckError> {
    fs::read_to_string(path).map_err(|e| MlCheckError::FileReadError {
        path: path.to_path_buf(),
        source: e,
    })
}

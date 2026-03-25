use serde::Deserialize;

use crate::error::MlCheckError;

#[derive(Deserialize)]
struct Notebook {
    cells: Vec<Cell>,
}

#[derive(Deserialize)]
struct Cell {
    cell_type: String,
    source: Vec<String>,
}

/// Extract Python source code from a Jupyter notebook JSON string.
/// Only includes code cells — markdown and raw cells are excluded.
/// This is a pure function (JSON deserialization, no file I/O).
pub fn extract_code_cells(json_str: &str, path: &std::path::Path) -> Result<String, MlCheckError> {
    let notebook: Notebook = serde_json::from_str(json_str).map_err(|e| {
        MlCheckError::NotebookParseError {
            path: path.to_path_buf(),
            source: e,
        }
    })?;

    let source: String = notebook
        .cells
        .iter()
        .filter(|cell| cell.cell_type == "code")
        .flat_map(|cell| cell.source.iter())
        .cloned()
        .collect();

    Ok(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn extracts_code_cells_only() {
        let json = r##"{
            "cells": [
                {"cell_type": "code", "source": ["import sklearn\n", "x = 1\n"], "metadata": {}, "outputs": [], "execution_count": null},
                {"cell_type": "markdown", "source": ["# This should be excluded\n"], "metadata": {}},
                {"cell_type": "code", "source": ["y = 2\n"], "metadata": {}, "outputs": [], "execution_count": null}
            ],
            "metadata": {},
            "nbformat": 4,
            "nbformat_minor": 4
        }"##;
        let result = extract_code_cells(json, Path::new("test.ipynb")).unwrap();
        assert!(result.contains("import sklearn"));
        assert!(result.contains("y = 2"));
        assert!(!result.contains("This should be excluded"));
    }

    #[test]
    fn empty_notebook_returns_empty_string() {
        let json = r#"{"cells": [], "metadata": {}, "nbformat": 4, "nbformat_minor": 4}"#;
        let result = extract_code_cells(json, Path::new("test.ipynb")).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn invalid_json_returns_error() {
        let result = extract_code_cells("not json", Path::new("test.ipynb"));
        assert!(result.is_err());
    }

    #[test]
    fn markdown_only_notebook_returns_empty() {
        let json = r#"{
            "cells": [
                {"cell_type": "markdown", "source": ["import sklearn\n"], "metadata": {}}
            ],
            "metadata": {},
            "nbformat": 4,
            "nbformat_minor": 4
        }"#;
        let result = extract_code_cells(json, Path::new("test.ipynb")).unwrap();
        assert!(!result.contains("sklearn"));
    }
}

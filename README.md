# mlcheck

`mlcheck` is a command-line tool that checks ML code for best practices. Think of it as a spell checker for machine learning workflows.

It scans Python (scikit-learn) and R (tidymodels) code to verify that common best practices are being followed: proper train/test splitting, stratification, reproducibility seeds, and data leakage prevention via pipelines or recipes.

## Checks

### Python / scikit-learn

| Check | What it looks for |
|-------|-------------------|
| Library import | `import sklearn` or `from sklearn ...` |
| Train/test split | `train_test_split()` |
| Stratification | `stratify=` parameter |
| Reproducibility | `random_state=` parameter |
| Data leakage prevention | `Pipeline()` or `make_pipeline()` |

### R / tidymodels

| Check | What it looks for |
|-------|-------------------|
| Library import | `library(tidymodels)` |
| Train/test split | `initial_split()` |
| Stratification | `strata=` parameter |
| Reproducibility | `set.seed()` |
| Data leakage prevention | `recipe()` |

All checks are **context-aware** -- patterns that appear only in comments are not counted as present.

## Supported file types

- `.py` -- Python scripts
- `.ipynb` -- Jupyter notebooks (only code cells are scanned; markdown cells are ignored)
- `.R` -- R scripts
- `.Rmd` -- R Markdown files

## Install

With [Rust and Cargo](https://www.rust-lang.org/tools/install) installed:

```sh
cargo install mlcheck
```

## Usage

Check a single file:

```sh
mlcheck --path path/to/your_file.py
```

Check all supported files in a directory:

```sh
mlcheck --path path/to/folder/
```

### Output formats

By default, results are printed to the console. You can also save results to CSV or SQLite:

```sh
# Save to CSV
mlcheck --path your_file.py --output csv

# Save to SQLite database
mlcheck --path your_file.py --output sql
```

Output files are written to your platform's data directory by default:

- **macOS**: `~/Library/Application Support/mlcheck/`
- **Linux**: `~/.local/share/mlcheck/`

You can override this with `--output-dir`:

```sh
mlcheck --path your_file.py --output csv --output-dir ./results
```

### Querying past results

To review all past checks stored in the SQLite database:

```sh
sqlite3 ~/Library/Application\ Support/mlcheck/mlcheck_output.db "SELECT * FROM mlcheck_results"
```

## Development

### Running tests

```sh
cargo test
```

### Mutation testing

[cargo-mutants](https://mutants.rs/) is used to verify test robustness:

```sh
cargo install cargo-mutants
cargo mutants
```

## Architecture

The codebase follows a **functional core, imperative shell** pattern:

- **Pure core** (`domain/`, `rules/`): types, check evaluation, pattern matching, scoring -- no I/O, fully deterministic and testable
- **I/O shell** (`scanner/`, `reporter/`, `cli.rs`, `lib.rs`): file reading, directory walking, console/CSV/SQLite output

Development follows **BDD dual-loop TDD**: integration tests define expected behavior from the outside in, unit tests drive internal implementation.

## Acknowledgements

The concept for this tool was in part inspired by the [statcheck](https://mbnuijten.com/statcheck/) project.

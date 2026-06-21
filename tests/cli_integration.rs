use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn given_python_file_with_all_practices_then_all_present() {
    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", "tests/fixtures/python/all_best_practices.py"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Imports the scikit-learn library: present"))
        .stdout(predicate::str::contains("Splits data into train and test sets: present"))
        .stdout(predicate::str::contains("Ensures class balance in train/test split: present"))
        .stdout(predicate::str::contains("Sets seed for reproducible results: present"))
        .stdout(predicate::str::contains("Uses a pipeline to guard against data leakage: present"))
        .stdout(predicate::str::contains("100%"));
}

#[test]
fn given_python_file_missing_practices_then_absent_shown() {
    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", "tests/fixtures/python/missing_practices.py"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Imports the scikit-learn library: present"))
        .stdout(predicate::str::contains("Splits data into train and test sets: absent"))
        .stdout(predicate::str::contains("Ensures class balance in train/test split: absent"))
        .stdout(predicate::str::contains("Sets seed for reproducible results: absent"))
        .stdout(predicate::str::contains("Uses a pipeline to guard against data leakage: absent"))
        .stdout(predicate::str::contains("20%"));
}

#[test]
fn given_python_file_with_only_comments_then_all_absent() {
    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", "tests/fixtures/python/commented_only.py"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Imports the scikit-learn library: absent"))
        .stdout(predicate::str::contains("Splits data into train and test sets: absent"))
        .stdout(predicate::str::contains("Ensures class balance in train/test split: absent"))
        .stdout(predicate::str::contains("Sets seed for reproducible results: absent"))
        .stdout(predicate::str::contains("Uses a pipeline to guard against data leakage: absent"))
        .stdout(predicate::str::contains("0%"));
}

#[test]
fn given_unsupported_extension_then_error() {
    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", "Cargo.toml"])
        .assert()
        .failure();
}

#[test]
fn given_nonexistent_path_then_error() {
    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", "nonexistent_file.py"])
        .assert()
        .failure();
}

// --- R / tidymodels integration tests ---

#[test]
fn given_r_file_with_all_practices_then_all_present() {
    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", "tests/fixtures/r/all_best_practices.R"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Imports the tidymodels library: present"))
        .stdout(predicate::str::contains("Splits data into train and test sets: present"))
        .stdout(predicate::str::contains("Ensures class balance in train/test split: present"))
        .stdout(predicate::str::contains("Sets seed for reproducible results: present"))
        .stdout(predicate::str::contains("Uses a recipe to guard against data leakage: present"))
        .stdout(predicate::str::contains("100%"));
}

#[test]
fn given_r_file_missing_practices_then_absent_shown() {
    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", "tests/fixtures/r/missing_practices.R"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Imports the tidymodels library: present"))
        .stdout(predicate::str::contains("Splits data into train and test sets: absent"))
        .stdout(predicate::str::contains("Ensures class balance in train/test split: absent"))
        .stdout(predicate::str::contains("Sets seed for reproducible results: absent"))
        .stdout(predicate::str::contains("Uses a recipe to guard against data leakage: absent"))
        .stdout(predicate::str::contains("20%"));
}

// --- Notebook integration tests ---

#[test]
fn given_notebook_with_all_practices_then_all_present() {
    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", "tests/fixtures/notebooks/all_best_practices.ipynb"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Imports the scikit-learn library: present"))
        .stdout(predicate::str::contains("Splits data into train and test sets: present"))
        .stdout(predicate::str::contains("Ensures class balance in train/test split: present"))
        .stdout(predicate::str::contains("Sets seed for reproducible results: present"))
        .stdout(predicate::str::contains("Uses a pipeline to guard against data leakage: present"))
        .stdout(predicate::str::contains("100%"));
}

#[test]
fn given_notebook_with_sklearn_only_in_markdown_then_all_absent() {
    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", "tests/fixtures/notebooks/sklearn_in_markdown_only.ipynb"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Imports the scikit-learn library: absent"))
        .stdout(predicate::str::contains("Splits data into train and test sets: absent"))
        .stdout(predicate::str::contains("Ensures class balance in train/test split: absent"))
        .stdout(predicate::str::contains("Sets seed for reproducible results: absent"))
        .stdout(predicate::str::contains("Uses a pipeline to guard against data leakage: absent"))
        .stdout(predicate::str::contains("0%"));
}

#[test]
fn given_r_file_with_only_comments_then_all_absent() {
    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", "tests/fixtures/r/commented_only.R"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Imports the tidymodels library: absent"))
        .stdout(predicate::str::contains("Splits data into train and test sets: absent"))
        .stdout(predicate::str::contains("Ensures class balance in train/test split: absent"))
        .stdout(predicate::str::contains("Sets seed for reproducible results: absent"))
        .stdout(predicate::str::contains("Uses a recipe to guard against data leakage: absent"))
        .stdout(predicate::str::contains("0%"));
}

// --- Directory scanning integration tests ---

#[test]
fn given_directory_with_mixed_files_then_all_checked() {
    let temp = assert_fs::TempDir::new().unwrap();

    temp.child("good.py").write_str(
        "from sklearn.model_selection import train_test_split\nX_train, X_test = train_test_split(X, y, stratify=y, random_state=42)\n"
    ).unwrap();

    temp.child("good.R").write_str(
        "library(tidymodels)\nset.seed(42)\n"
    ).unwrap();

    // This .txt file should be silently ignored
    temp.child("notes.txt").write_str("some notes").unwrap();

    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", temp.path().to_str().unwrap()])
        .assert()
        .success()
        // Python file results
        .stdout(predicate::str::contains("Imports the scikit-learn library: present"))
        // R file results
        .stdout(predicate::str::contains("Imports the tidymodels library: present"));
}

#[test]
fn given_empty_directory_then_error() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("readme.txt").write_str("nothing useful").unwrap();

    Command::cargo_bin("mlcheck")
        .unwrap()
        .args(["--path", temp.path().to_str().unwrap()])
        .assert()
        .failure();
}

// --- CSV output integration tests ---

#[test]
fn given_csv_output_then_csv_file_created_with_results() {
    let temp = assert_fs::TempDir::new().unwrap();
    let csv_path = temp.child("mlcheck_output.csv");

    Command::cargo_bin("mlcheck")
        .unwrap()
        .args([
            "--path", "tests/fixtures/python/all_best_practices.py",
            "--output", "csv",
            "--output-dir", temp.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    let csv_content = std::fs::read_to_string(csv_path.path()).unwrap();
    assert!(csv_content.contains("file_name,"));
    assert!(csv_content.contains("all_best_practices.py"));
    assert!(csv_content.contains("true"));
    // Should have header + 5 data rows
    assert_eq!(csv_content.lines().count(), 6);
}

// --- SQLite output integration tests ---

#[test]
fn given_sql_output_then_database_created_with_results() {
    let temp = assert_fs::TempDir::new().unwrap();
    let db_path = temp.child("mlcheck_output.db");

    Command::cargo_bin("mlcheck")
        .unwrap()
        .args([
            "--path", "tests/fixtures/python/all_best_practices.py",
            "--output", "sql",
            "--output-dir", temp.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    // Verify the database file exists and has the right number of rows
    assert!(db_path.path().exists());
    let conn = rusqlite::Connection::open(db_path.path()).unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM mlcheck_results", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 5);
}

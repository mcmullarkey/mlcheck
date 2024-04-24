# mlcheck

`mlcheck` is a command line tool to check for ML best practices in different coding documents.

Think of this tool as a spell-check equivalent for ML best practices.

The current version can detect `scikit-learn` style Python code in .py or .ipynb (Jupyter Notebook) files.

# Install

If you have Rust and Cargo installed (see <a href="https://www.rust-lang.org/tools/install" target="_blank">this resource</a> if you haven't), you can install `mlcheck` from <a href="https://crates.io/" target="_blank">crates.io</a> using:

`cargo install mlcheck`

# Running mlcheck

To run `mlcheck` on a file you can run the following terminal command:

`mlcheck --path path/to/your_file_name.py`

To run `mlcheck` on a folder with .py and/or .ipynb files you can run the following terminal command:

`mlcheck --path path/to/folder/`

# Analyzing mlcheck results

To look back at all the past checks you've run using mlcheck you can query the mlcheck_output.db `sqlite` database that's automatically created when you run mlcheck for the first time. As long as you run `mlcheck` in the same folder, new checks will be appended to the database.

`sqlite3 mlcheckoutput.db`

`sqlite> select * from mlcheck_results`

If you'd prefer to save your `mlcheck` results to a csv, run your commands like this

`mlcheck --path path/to/your_file_name.py --output csv`

# Disclaimer

Note: `mlcheck` is at an incredibly early stage and is under active development. Breaking changes are likely.

# Acknowledgements

The concept for this tool was in part inspired by the <a href="https://mbnuijten.com/statcheck/" target="_blank">statcheck</a> project.

# Potential future features

- Able to identify tensorflow style code
- Able to identify keras style code
- Ability to identify tidymodels style R code across .R, .Rmd, and .qmd file types
- Add more specific, sophisticated regex across styles

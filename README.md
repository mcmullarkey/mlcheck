# mlcheck

`mlcheck` is a command line tool to check for ML best practices in different coding documents.

This tool is primarily intended as a checklist (or a spell-check equiavlent for ML best practices) for your own ML coding files.

This tool was in part inspired by the <a href="https://mbnuijten.com/statcheck/" target="_blank">statcheck</a> project.

If you have Rust and Cargo installed (see <a href="https://www.rust-lang.org/tools/install" target="_blank">this resource</a> if you haven't), you can install `mlcheck` from <a href="https://crates.io/" target="_blank">crates.io</a> using:

`cargo install mlcheck`

To run `mlcheck` on a file you can run the following terminal command:

`mlcheck --path path/to/your_file_name.py`

Note: `mlcheck` is at an incredibly early stage and is under active development. Breaking changes are likely.

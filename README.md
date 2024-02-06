# Template for Devloping a Rust library within Python application

`cargo test --no-default-features` # Run tests without default features (i.e. *pyo3/extension-module*)

`rust-gdb --args python -c "import my_package; my_package.sum_to_string(1, 2)"`

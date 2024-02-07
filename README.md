# Template for Devloping a Rust library within Python application



## Calling Python from Rust

Python library must be added to `LD_LIBRARY_PATH` or `DYLD_LIBRARY_PATH` manually

```bash
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/someone/codes/pyrust-template/.conda/lib
```

Use `auto-initialize` feature to initialize Python interpreter automatically `pyo3 = { version = "0.20.2", features = [ "auto-initialize",] }`

## Extension Module

`features = ["extension-module"]` will skip linking agains `libpython.so`, thus, cargo build/run/test will fail.
Linking to `libpython.so` is needed for binaries, tests and examples "just work"

There are 4 way to enable extension-module feature:

1. In Cargo.toml
   ```toml
   [dependencies]
   pyo3 = { version = "0.20.2", features = ["extension-module"] }
   ```
2. In pyproject.toml
   ```toml
    [tool.maturin]
    python-source = "python"
    features = ["pyo3/extension-module"]
   ```
3. Make it optional, then run `maturin develop --release --strip` to build the extension module
   ```toml
   [dependencies]
   pyo3 = { version = "0.20.2" }
   ```
4. Make it default, then disable it by `cargo build --no-default-features`
   ```toml
    [dependencies.pyo3]
    version = "0.20.2"

    [features]
    extension-module = ["pyo3/extension-module"]
    default = ["extension-module"]
   ```

**NOTE**: to build for binary and tests use both `cdylib` and `rlib` option (i.e. to run `cargo test` must add `rlib`)
```toml
[lib]
name = "pyrust_py"
crate-type = ["cdylib", "rlib"]
```

## Other build configuration

- python `abi3` limited  Python API which allows wheels to be used on multiple Python versions

## Debug

- `rust-gdb --args python -c "import my_package; my_package.sum_to_string(1, 2)"`

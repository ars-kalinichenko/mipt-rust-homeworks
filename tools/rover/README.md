# Rover

Helper tool for the Rust language course.

## The most useful

- `rover test` - tests current problem. It supposes you're on its root and works the same way as the CI grader.
- `rover submit` - submits current problem. It supposes you're on its root. The course repository must set up a `solutions` folder with the student's git repository.

## Technical

### Structure of repository

The name of the course repository must be `mipt-rust` or `mipt-rust-private`. Any problem must be located at its `problems/GROUP/TITLE` folder. There must be `.config.yml` at the problem root.

```plain
mipt-rust
├── problems
│   ├── GROUP1
│   │   └── TITLE1
│   │       ├── ...
│   │       └── .config.yml
│   ├── GROUP2
│   │   └── ...
│   │
```

### Problem config

Rover relies on `.config.yml` that should be placed in the root of the problem the following format:

```yml
toolchain: stable
allowed-patterns:
  - src/lib.rs
steps:
  linters:
    - forbid-unsafe
    - cargo-fmt
    - cargo-clippy
  testing:
    - cargo-test
```

- Toolchains:
  - `stable` - just regular stable toolchain.
  - `nightly` - latest nightly toolchain.
- Allowed patterns:
  - Here might be any problem subpath and even any Unix shell style pattern. All matched paths **must be files**.
- Steps - represents the steps of testing. They may be named how you like, `rover` will launch them step-by-step.
  - Commands - part of a step:
    - `forbid-unsafe` - checks if `#![forbid(unsafe_code)]` line present in all allowed user files.
    - `cargo-fmt` - launches `cargo fmt` in the root of the problem with the current toolchain.
    - `cargo-clippy` - launches `cargo clippy` in the root of the problem with the current toolchain.
    - `cargo-test` - launches `cargo test` in the root of the problem with the current toolchain.
    - `python-test` - launches `python3 test.py` at the root of problem directory and checks the exit code.
    - `forbid-collections` - bans solution if there's any collection from `std::collections`.

### Compose config

To create from your solutions repository a public one, you can use `compose` utility. The config `.compose.yml` should be stored at the root of the repository:

```yml
problems:
  - tutorial/add
  - intro/combinations
  - intro/conway
  - intro/min-queue
  - std-collections/comm
  - std-collections/mpsc
  - std-collections/prefix
  - std-collections/lru-cache
tools:
  - rover
copy:
  - docs
  - lectures
  - .deadlines.yml
  - .gitignore
  - .grader-ci.yml
  - LICENSE
  - README.md
skip-entries:
  - lectures/lecture-02
do-not-delete:
  - .git
```

- `problems` - list of problems from `problems` directory. The utility will copy them and add them to `Cargo.toml` at the new root.
- `tools` - list of tools from `tools` directory. The utility will copy them and add them to `Cargo.toml` at the new root.
- `copy` - entries that will be copied to the destination.
- `skip-entries` - the entries that will be deleted after tool finishes dealing with `problems`, `tools`, and `skip-entries`.
- `do-not-delete` - by default, the output folder is just emptied, but you can spare some folders or files. **Note these entries must be simply filenames of directory names**.

### Compose commands

There are some simple commands to hide your code. Before:

```rust
/* ----- */
// compose::begin_private
fn just_hidden() {}
// compose::end_private
/* ----- */
// compose::begin_private(no_hint)
fn erased_completely() {}
// compose::end_private
/* ----- */
fn unimpl() -> i32 {
  // compose::begin_private(unimplemented)
  42
  // compose::end_private
}
/* ----- */
```

After:

```rust
/* ----- */
// TODO: your code goes here.
/* ----- */
/* ----- */
fn unimpl() -> i32 {
  // TODO: your code goes here.
  unimplemented!()
}
/* ----- */
```

### Commands

- `rover test --path PATH --move-files REPO --checkout-branch --report-to CI`
  - If `REPO` is set, checkout branch `GROUP/TITLE` if option `--checkout-branch` present, and then copy the files from the solutions repository.
  - `PATH` is the root of the problem.
  - `CI` is the string that represents the CI name. Supported ones are:
    - `no-report` (default) - just don't report anything.
    - `manytask` - send the result to the Manytask of the School of Data Analysis.
- `rover submit --path PATH --solutions-repo REPO --message MSG`
  - Gets problem from `PATH`. The default `PATH` is the current directory.
  - Changes branch to problem `GROUP/TITLE`, adds files to git, commits with message `MSG`, and pushes to remote. If `MSG` is not set, use some default one).
  - Copies allowed files to `REPO`. Otherwise, it uses `solutions` folder from the course repository.
- `rover compose --input INPUT --output OUTPUT`
  - Reads `.compose.yml` from `INPUT` repository.
  - Prunes `OUTPUT` directory, ignoring files from `do-not-delete`.
  - Copies problems and processes sources.
  - Copies tools and processes sources.
  - Copies files to copy.
  - Deletes entries from `skip-entries`.

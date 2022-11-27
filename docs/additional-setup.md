# Additional setup

Some of the problems, which are closer to the end of the course, require the installation of additional software to test and run. Most likely you will open this document as a link when reading the problem's statement. Just find a needed guide here or click the link on the statement.

## Release builds

There're some problems that need the release build with optimizations, since without them the data structure or the algorithm work too slow. To produce a release build, add a flag `--release`:

```sh
cargo test --release
```

## Nightly compiler

Some of the features are not available in stable Rust since they're in the development and testing stage. However, sometimes we want to check them out in our course! To install the nightly compiler, run:

```sh
rustup install nightly
```

To make cargo use the nightly compiler, run:

```sh
cargo +nightly test
```

## Miri interpreter

When writing unsafe code, you'll want to check it for memory unsafety and undefined behavior. One of the tools to help us is [Miri](https://github.com/rust-lang/miri). Don't forget to install [nightly compiler](#nightly-compiler)!

```sh
rustup +nightly component add miri
cargo install xargo
```

To make cargo use Miri, run:

```sh
cargo +nightly miri test # To run tests with MIRI
```

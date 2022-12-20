# simplyR

A Rust implementation of the BEST matching algorithm.

[![CI](https://github.com/BESTenergytrade/simplyR/actions/workflows/ci.yml/badge.svg)](https://github.com/BESTenergytrade/simplyR/actions/workflows/ci.yml)

## Installation

* For Ubuntu 20.04 and higher, you need to install some dependencies:

```sh
sudo apt install git build-essential
```

* Install the latest stable version of Rust (at least version 1.66.0), e.g. via <https://rustup.rs/>

* Get the code

```sh
git clone ...
cd simplyR
```

* Compile, run

```sh
# Build and run
cargo run
# Build only
cargo build
# Run tests
cargo test
```

## Running

The program accepts a file path to a JSON file as the first argument. See
`example_market_input.json` for an example file.

You can either compile the program and access the binary directly (it's in the
`target` directory):

```sh
cargo build --release
target/release/simplyr example_market_input.json
```

Or you can compile and run in one invocation:

```sh
cargo run --release -- example_market_input.json
```

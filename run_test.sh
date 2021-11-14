#!/bin/bash

set -eu

cargo build --release

cd tools
../../target/release/a < in/0000.txt > out/0000.txt
cargo run --release --bin vis in/0000.txt out/0000.txt

#!/bin/bash

set -e

if [[ $1 ]]; then
    test_num="$1";
else
    test_num="0000";
fi

cargo build --release

cd tools
../../target/release/a < in/${test_num}.txt > out/${test_num}.txt
cargo run --release --bin vis in/${test_num}.txt out/${test_num}.txt

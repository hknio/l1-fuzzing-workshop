#!/bin/bash
cargo fuzz coverage --fuzz-dir . --release fuzzer

# Determine the correct path based on the OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    TARGET_TRIPLE="x86_64-unknown-linux-gnu"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    TARGET_TRIPLE="aarch64-apple-darwin"
else
    echo "Unsupported OS: $OSTYPE"
    exit 1
fi

llvm-cov show "./target/$TARGET_TRIPLE/coverage/$TARGET_TRIPLE/release/fuzzer" \
    --format=html \
    -instr-profile=./coverage/fuzzer/coverage.profdata \
    --output-dir=./coverage/report \
    -show-directory-coverage \
    --show-instantiations=false

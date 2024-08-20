#!/bin/bash
cargo fuzz run --fuzz-dir . --release --features fuzzing,libfuzzer --sanitizer=none fuzzer

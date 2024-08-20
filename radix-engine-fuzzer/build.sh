#!/bin/bash
cargo fuzz build --fuzz-dir . --release --sanitizer=none --features fuzzing,libfuzzer

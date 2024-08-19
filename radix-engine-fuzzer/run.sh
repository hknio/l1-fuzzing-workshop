#!/bin/bash
cargo fuzz run --fuzz-dir . --release --features fuzzing --sanitizer=none fuzzer

#!/bin/bash
cargo fuzz build --fuzz-dir . --release --features fuzzing

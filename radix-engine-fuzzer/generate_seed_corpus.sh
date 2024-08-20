#!/bin/bash

DUMP_MANIFEST_DIR="`pwd`/corpus/fuzzer"
mkdir -p "$DUMP_MANIFEST_DIR"
DUMP_MANIFEST_DIR="$DUMP_MANIFEST_DIR" cargo test -p radix-engine-tests --no-fail-fast
cat "$DUMP_MANIFEST_DIR"/*.manifest > "$DUMP_MANIFEST_DIR"/../../fuzz_targets/merged_seed_corpus.bin

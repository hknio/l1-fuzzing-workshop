#!/bin/bash

DUMP_MANIFEST_DIR="`pwd`/corpus/fuzzer"
mkdir -p "$DUMP_MANIFEST_DIR"
DUMP_MANIFEST_DIR="$DUMP_MANIFEST_DIR" cargo test -p radix-engine-tests --no-fail-fast
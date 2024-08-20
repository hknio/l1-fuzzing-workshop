#!/bin/bash
 cargo afl build --features=fuzzing,afl --release  
 mkdir -p artifacts/afl
 AFL_FORKSRV_INIT_TMOUT=120000 cargo afl fuzz -i corpus/fuzzer/ -o artifacts/afl ../target/release/fuzzer
 
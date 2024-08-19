#!/bin/bash
# requries perf and https://github.com/jrfonseca/gprof2dot

sudo perf record --call-graph=dwarf -p `pgrep fuzzer` sleep 10
perf script | c++filt | python3 -m gprof2dot -f perf -n 5.0 -e 3.0 | dot -Tpng -o output.png

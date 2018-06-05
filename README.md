# gcodetools

[![Build Status](https://travis-ci.org/jamwaffles/gcodetools.svg?branch=master)](https://travis-ci.org/jamwaffles/gcodetools)

Libraries for working with CNC GCode

Note to self for profiling:

```bash
# Cargo.toml
# [profile.release]
# debug = true

cargo test --release -- --nocapture
valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes ../target/release/deps/universal_gcode_sender_suite-5ddc65b6ce289fa6
```
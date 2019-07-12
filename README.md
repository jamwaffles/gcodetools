# gcodetools

[![Build Status](https://travis-ci.org/jamwaffles/gcodetools.svg?branch=master)](https://travis-ci.org/jamwaffles/gcodetools)
[![codecov](https://codecov.io/gh/jamwaffles/gcodetools/branch/master/graph/badge.svg)](https://codecov.io/gh/jamwaffles/gcodetools)

Libraries for working with CNC GCode

To profile tests:

```bash
# In top level Cargo.toml
# [profile.dev]
# opt-level = 3

# Any suite in tests/ NOT in release mode
cargo test --test tinyg_suite --features profile -- --nocapture

pprof --web ../target/debug/trajectory_planner-02630d1462f0a1b5 ../target/profiles/trajectory_planner-02630d1462f0a1b5.profile
```

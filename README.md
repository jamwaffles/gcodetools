# gcodetools

[![Build Status](https://travis-ci.org/jamwaffles/gcodetools.svg?branch=master)](https://travis-ci.org/jamwaffles/gcodetools)

Libraries for working with CNC GCode

To profile tests:

```bash
# In top level Cargo.toml
# [profile.dev]
# opt-level = 3

# Any suite in tests/ NOT in release mode
cargo test --test tinyg_suite --features profile -- --nocapture

./analyse-profile.sh <path to test bin>
```

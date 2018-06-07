# gcodetools

[![Build Status](https://travis-ci.org/jamwaffles/gcodetools.svg?branch=master)](https://travis-ci.org/jamwaffles/gcodetools)

Libraries for working with CNC GCode

To profile tests:

```bash
brew install go gprof2dot
go get -u github.com/google/pprof

# Any suite in tests/
cargo test --test tinyg_suite --features profile -- --nocapture

pprof --callgrind ../target/debug/deps/<binary name> ./profiles/<profile name>.profile > ./profiles/profile.callgrind
gprof2dot --format=callgrind --output=./profiles/profile.dot ./profiles/profile.callgrind
dot -Tpng ./profiles/profile.dot -o ./profiles/graph.png

# MacOS only
open ./profiles/graph.png
```

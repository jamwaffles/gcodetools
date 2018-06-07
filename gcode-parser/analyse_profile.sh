#!/bin/sh

# Make sure the deps are installed:
# brew install go gprof2dot
# go get -u github.com/google/pprof

BIN_PATH="$1"

BIN_NAME=$(basename -- "$BIN_PATH")
BIN_NAME="${BIN_NAME%.*}"

pprof --callgrind $BIN_PATH "./profiles/${BIN_NAME}.profile" > "./profiles/${BIN_NAME}.callgrind"
gprof2dot --format=callgrind --output="./profiles/${BIN_NAME}.dot" "./profiles/${BIN_NAME}.callgrind"
dot -Tpng "./profiles/${BIN_NAME}.dot" -o "./profiles/${BIN_NAME}.png"

# MacOS only
open "./profiles/${BIN_NAME}.png"

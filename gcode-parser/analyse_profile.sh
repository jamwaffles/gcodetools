#!/bin/sh

# Make sure the deps are installed:
# Mac:
# brew install go gprof2dot
# Linux/WSL:
# sudo apt-get install -y libgoogle-perftools-dev golang python graphviz python-pip libunwind-dev
# pip install gprof2dot
#
# go get -u github.com/google/pprof

BIN_PATH="$1"

BIN_NAME=$(basename -- "$BIN_PATH")
BIN_NAME="${BIN_NAME%.*}"

echo "Pprof..."
pprof --callgrind $BIN_PATH "./profiles/${BIN_NAME}.profile" > "./profiles/${BIN_NAME}.callgrind"
echo "Gprof2dot..."
gprof2dot --format=callgrind --output="./profiles/${BIN_NAME}.dot" "./profiles/${BIN_NAME}.callgrind"
echo "Dot..."
dot -Tpng "./profiles/${BIN_NAME}.dot" -o "./profiles/${BIN_NAME}.png"

echo "Done"

# MacOS only
# open "./profiles/${BIN_NAME}.png"

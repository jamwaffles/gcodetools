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
pprof --nodecount=500 --nodefraction=0.0001 --edgefraction=0.0001 --maxdegree=128 --alloc_space --callgrind $BIN_PATH "./profiles/${BIN_NAME}.profile" > "./profiles/${BIN_NAME}.callgrind"
echo "Gprof2dot..."
gprof2dot --show-samples -n 0.01 -e 0.001 --format=callgrind --output="./profiles/${BIN_NAME}.dot" "./profiles/${BIN_NAME}.callgrind"
echo "Dot..."
dot -Tpng "./profiles/${BIN_NAME}.dot" -o "./profiles/${BIN_NAME}.png"

echo "Done"

# MacOS only
open "./profiles/${BIN_NAME}.png"

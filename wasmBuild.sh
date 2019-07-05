#!/bin/bash

set -e

echo "============================================="
echo "Compiling wasm"
echo "============================================="

wasm-pack build --target web
rm pkg/.gitignore
rm pkg/README.md

echo "============================================="
echo "Compiling wasm  done"

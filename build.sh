#!/bin/bash

set -e

echo "============================================="
echo "Compiling wasm"
echo "============================================="

wasm-pack build --target web
rm pkg/.gitignore

echo "============================================="
echo "Compiling wasm  done"

#!/bin/bash

set -e

echo "============================================="
echo "Compiling wasm"
echo "============================================="
(
rustup run nightly \
  cargo build \
  --target wasm32-unknown-unknown \
  --release
  cp target/wasm32-unknown-unknown/release/wasmboypluginhqx.wasm dist/
  )
  echo "============================================="
  echo "Compiling wasm  done"

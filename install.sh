#!/bin/bash

# Add the nightly toolchain
rustup toolchain install nightly

# Add our toolchains target
rustup target add --toolchain nightly wasm32-unknown-unknown

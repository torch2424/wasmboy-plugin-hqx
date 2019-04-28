# wasmboy-plugin-hqx
Plugin for WasmBoy to add support for hqx upscaling. Built with Rust for WebAssembly.

# Contributing

Depends on the `hqx` crate, within [wasmboy-rs](https://github.com/CryZe/wasmboy-rs).

This project uses `wasm-pack`, and is scaffolded follwing the [wasm-pack npm package guide](https://rustwasm.github.io/wasm-pack/book/tutorials/npm-browser-packages/index.html).

Just for personal reference, after reading wasm pack, and seeing it's JS output, it got me wondering if I need `wasm-pack`? Then I read [this reddit post](https://www.reddit.com/r/rust/comments/a8cixt/webassembly_without_node/), and this [reddit guide to JUST get a wasm module](https://www.reddit.com/r/rust/comments/9t95fd/howto_setting_up_webassembly_on_stable_rust/). Made me realize using `wasm-pack`, even just for small projects that you kind of just want a wasm file, and instead play with the JS output to do what you want. This is because looking at the minimal guide, it gets really funny with types, and memory. Thus wasm-pack abstracts that to make it feel like you are writing rust, and not just trying to abstract the JS platform and try to expose it to you.

## Wasm

Wasm is what does the actual upscaling logic.

* Build and output to `pkg/`: `wasm-pack build`

## Lib



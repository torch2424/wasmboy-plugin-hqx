# wasmboy-plugin-hqx

Plugin for WasmBoy to add support for hqx upscaling. Built with Rust for WebAssembly. This plugin uses the [hqx crate](https://github.com/CryZe/wasmboy-rs/tree/master/hqx) from [wasmboy-rs](https://github.com/CryZe/wasmboy-rs).

[Demo]()

![WasmBoy Hq3x Example](./assets/wasmboyPluginHqx.png)

# Usage

Install the plugin in your WasmBoy project:

`npm install --save wasmboy-plugin-hqx`

Use the plugin in your project:

```javascript
import {WasmBoy} from 'wasmboy';
import wasmboyHqx from '../lib/lib';

const playROM = async () => {
  WasmBoy.addPlugin(wasmboyHqx());
  await WasmBoy.config({});
  WasmBoy.setCanvas(canvasElement);

  await WasmBoy.loadROM(GameROMs.TobuTobuGirl);
  awaut WasmBoy.play
};
playROM()
```

See the [demo source code](./demo) as a simple example.

# Caveats

There are some caveats / things to be aware of when using this plugin:

## Usage on respective platforms

Currently, this plugin is **only meant for use in a web browser**. The reason being, this plugin needs to change the canvas size. Also, as of 0.5.0, WasmBoy kind of has a small feature set of the node / "headless" implementation. Though, I am completely open to adding node support.

## Using other Graphical WasmBoy Plugins

Users who want to use this plugin with other plugins, **should be careful with using other graphical based plugins, and/or the order of the graphical based plugins, for the following reasons:**

* This uses the `canvas` callback of the WasmBoy Plugin API, to modify the canvas to the correct size to handle the hqx scaling. Thus, **This plugin should probably be added last compared to your other graphical wasmboy plugins**.

* In the `graphics` callback, this returns an expanded `imageDataArray` that contains the upscaled image. Thus, **This plugin should probably be added last compared to your other graphical wasmboy plugins**.

# Contributing

This projects is your standard JS Workflow, using [rollup](https://rollupjs.org/guide/en/) as the bundler for the lib and demo. For the Rust/Wasm workflow, this project uses `wasm-pack`. And, depends on the `hqx` crate, within [wasmboy-rs](https://github.com/CryZe/wasmboy-rs).

For normal development, you want to use the command: `npm run dev`.

For building the entire project, you want to use the command: `npm run build`.

## Wasm

Source code is within the `src/` directory, and output is given in the `pkg`. Wasm is what does the actual upscaling logic. Depends on the `hqx` crate, within [wasmboy-rs](https://github.com/CryZe/wasmboy-rs). Follows the ideas of [WebAssembly Linear Memory](https://wasmbyexample.dev/examples/webassembly-linear-memory/webassembly-linear-memory.rust.en-us.html) from [Wasm By Example](https://wasmbyexample.dev/). 

You can build the wasm by running: `npm run wasm:build`.

## Lib

Source code is within the `lib/` directory. The lib wraps around the Wasm module, and exports the actual WasmBoy plugin object. 

You can build the lib by running: `npm run lib:build`;

## Demo

Source code is within the `demo/` directory. This is a super small [preact](https://preactjs.com/) app that has a single component that allows playing [Tobu Tobu Girl](http://tangramgames.dk/tobutobugirl/) using WasmBoy, and this plugin.

You can build the demo by running: `npm run demo:build`;

# Special Thanks

* CryZe - for making wasmboy-rs, and the awesome hqx crate.

* Tangram Games - for making an awesome Homebrew GB Game, [Tobu Tobu Girl](http://tangramgames.dk/tobutobugirl/)


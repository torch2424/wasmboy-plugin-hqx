// Our required rust wasm module
import wasmModuleUrl from '../dist/wasmboypluginhqx.wasm';

const GAMEBOY_CAMERA_WIDTH = 160;
const GAMEBOY_CAMERA_HEIGHT = 144;

const importObject = {};
const wasmBrowserInstantiate = async wasmModuleUrl => {
  let response = undefined;

  // Safari does not support .instantiateStreaming()
  // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/instantiateStreaming
  if (WebAssembly.instantiateStreaming) {
    response = await WebAssembly.instantiateStreaming(fetch(wasmModuleUrl), importObject);
  } else {
    const fetchAndInstantiateTask = async () => {
      const wasmArrayBuffer = await fetch(wasmModuleUrl).then(response => response.arrayBuffer());
      return WebAssembly.instantiate(wasmArrayBuffer, importObject);
    };
    response = await fetchAndInstantiateTask();
  }

  return response;
};

export default function (passedUpscaleFactor) {

  let wasmModule = undefined;
  let wasmMemory = undefined;

  const init = async () => {
    wasmModule = await wasmBrowserInstantiate(wasmModuleUrl);
    console.log(wasmModule.instance.exports);

    wasmMemory = new Uint8Array(wasmModule.instance.exports.memory.buffer);
  };
  init();

  // Get our upscale factor.
  const upscaleFactor = 3;
  if (passedUpscaleFactor && passedUpscaleFactor >= 2 && passedUpscaleFactor <= 4) {
    upscaleFactor = passedUpscaleFactor;
  }

  return {
    name: 'wasmboy-plugin-hqx',
    canvas: (canvasElement, canvasContext, canvasImageData) => {
      console.log('setCanvas hook', canvasElement, canvasContext, canvasImageData);

      // Set our canvas to match our upscaleFactor
      const upscaleWidth = GAMEBOY_CAMERA_WIDTH * upscaleFactor;
      const upscaleHeight = GAMEBOY_CAMERA_HEIGHT * upscaleFactor;
      canvasElement.width = upscaleWidth;
      canvasElement.height = upscaleHeight;
      canvasImageData = canvasContext.createImageData(upscaleWidth, upscaleHeight);

      canvasElement.style = '';

      return {
        canvasElement,
        canvasImageData,
        canvasContext
      }
    },
    graphics: (imageDataArray) => {
      if (!wasmModule) {
        return;
      }

      console.log('start');

      console.log('yooo', wasmModule.instance.exports.hqx());
      console.log('output', wasmModule.instance.exports.get_output_buffer_pointer());

      const outputPointer = wasmModule.instance.exports.get_output_buffer_pointer();

      wasmMemory[outputPointer + 0] = 125;
      console.log('please?', wasmModule.instance.exports.get_output_index(0), wasmMemory[outputPointer + 0]);

      console.log('output', wasmModule.instance.exports.get_output_buffer_pointer());

      console.log('set', wasmModule.instance.exports.set_output_index(1, 104));
      console.log('last test', wasmMemory[outputPointer + 1]);
    }
  }
}

// Our required rust wasm module
import wasmModuleUrl from '../pkg/wasmboypluginhqx_bg.wasm';

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


  const init = async () => {
    wasmModule = await wasmBrowserInstantiate(wasmModuleUrl);
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

      // Get our output array
      let wasmMemory = new Uint8Array(wasmModule.instance.exports.memory.buffer);
      // Get our pointers
      const inputBuffer = {
        pointer: wasmModule.instance.exports.get_input_buffer_pointer(),
        size: wasmModule.instance.exports.get_input_buffer_size()
      };

      // Fill the input buffer
      // It is a 32 bit buffer
      // https://github.com/CryZe/wasmboy-rs/blob/master/src/main.rs#L233
      for(let i = 0; i < inputBuffer.size; i++) {
        const pixelBase = i * 4;

        const red = imageDataArray[pixelBase + 0];
        const green = imageDataArray[pixelBase + 1];
        const blue = imageDataArray[pixelBase + 2];
        const alpha = 255;

        wasmMemory[inputBuffer.pointer + pixelBase + 0] = blue;
        wasmMemory[inputBuffer.pointer + pixelBase + 1] = green;
        wasmMemory[inputBuffer.pointer + pixelBase + 2] = red;
        wasmMemory[inputBuffer.pointer + pixelBase + 3] = alpha;
      } 
      
      // Run the hqx
      wasmModule.instance.exports.hqx();

      // Get our output array
      const outputBuffer = {
        pointer: wasmModule.instance.exports.get_output_buffer_pointer(),
        size: wasmModule.instance.exports.get_output_buffer_size()
      };
      wasmMemory = new Uint8Array(wasmModule.instance.exports.memory.buffer);

      // Retreive our output buffer
      const response = new Uint8ClampedArray(outputBuffer.size);
      for (let i = 0; i < outputBuffer.size; i++) {
        response[i] = wasmMemory[outputBuffer.pointer + i];
      }

      return response;
    }
  }
}

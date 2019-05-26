// Our required rust wasm module
import rustWasmInit from '../pkg/wasmboy_plugin_hqx';
import wasmModuleUrl from '../pkg/wasmboy_plugin_hqx_bg.wasm';

const GAMEBOY_CAMERA_WIDTH = 160;
const GAMEBOY_CAMERA_HEIGHT = 144;

export default function (passedUpscaleFactor) {

  console.log('wasmboy-plugin-hqx lib', rustWasmInit);

  let wasmModule = undefined;
  let wasmMemory = undefined

  const init = async () => {
    const response = await rustWasmInit(wasmModuleUrl);
    console.log('plugin init response:', response);
    wasmModule = response;
    console.log(wasmModule);
    wasmMemory = new Uint8Array(wasmModule.memory);
  };
  init();

  // Get our upscale factor.
  const upscaleFactor = 3;
  if (passedUpscaleFactor && passedUpscaleFactor >= 2 && passedUpscaleFactor <= 3) {
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
    },
    graphics: (imageDataArray) => {
      if (!wasmModule) {
        return;
      }

      const rgbaPointer = wasmModule.hqx(imageDataArray);
    }
  }
}

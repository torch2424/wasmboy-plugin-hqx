// Our required rust wasm module
import rustWasmInit from '../pkg/wasmboy_plugin_hqx';
import wasmModuleUrl from '../pkg/wasmboy_plugin_hqx_bg.wasm';

const GAMEBOY_CAMERA_WIDTH = 160;
const GAMEBOY_CAMERA_HEIGHT = 144;

export default function (passedUpscaleFactor) {

  console.log('wasmboy-plugin-hqx lib', rustWasmInit);

  let wasmModule = undefined;
  let uint8WasmMemory = undefined;
  let uint32WasmMemory = undefined;

  const init = async () => {
    const response = await rustWasmInit(wasmModuleUrl);
    console.log('plugin init response:', response);
    wasmModule = response;
    console.log(wasmModule);
    uint8WasmMemory = new Uint8Array(wasmModule.memory);
    uint32WasmMemory = new Uint32Array(wasmModule.memory);
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

      // Fill our intput buffer
      const inputArrayPointer = wasmModule.get_pointer_to_32_bit_input_buffer();
      console.log('argb pointer', inputArrayPointer);
      for (let i = 0; i < imageDataArray.length; i += 4) {
        const alpha = 0xFF;
        const red = imageDataArray[0] << 16;
        const green = imageDataArray[1] << 8;
        const blue = imageDataArray[2];
        
        uint32WasmMemory[inputArrayPointer + i] = alpha | red | green | blue;
      }
      
      // Get the pointer to our new hqx rgba
      const rgbaPointer = wasmModule.hqx(imageDataArray);

      console.log('rgba pointer', rgbaPointer);
      
      // Reset our image data to a larger array
      const response = new Uint8ClampedArray(
        GAMEBOY_CAMERA_WIDTH * GAMEBOY_CAMERA_HEIGHT * upscaleFactor * 4
      );
      
      // Set all the values to the array
      for (let i = 0; i < response.length; i++) {
        response[i] = uint8WasmMemory[rgbaPointer + i];
      }

      return response;
    }
  }
}

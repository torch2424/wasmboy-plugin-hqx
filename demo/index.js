import { h, render, Component } from 'preact';

import {WasmBoy} from '../../wasmboy/dist/wasmboy.wasm.esm';

import './index.css';

import wasmboyHqx from '../lib/index';

// Import our open source roms
import { GameROMs } from 'shared-gb/openSourceROMs/urlImports';

console.log('wasmboy-plugin-hqx demo');
console.log('WasmBoy', WasmBoy);

// Hotkeys
WasmBoy.ResponsiveGamepad.onInputsChange(
  [
    WasmBoy.ResponsiveGamepad.RESPONSIVE_GAMEPAD_INPUTS.SPECIAL
  ],
  state => {
    // Play / Pause
    if (WasmBoy.isPlaying() && state.SPECIAL) {
      WasmBoy.pause();
    } else if (!WasmBoy.isPlaying() && state.SPECIAL) {
      WasmBoy.play();
    }
  }
);

class PluginDemo extends Component {
  componentDidMount() {
    // Get our HTML5 Canvas element
    const canvasElement = document.querySelector('canvas');

    const playROMTask = async () => {
      WasmBoy.addPlugin(wasmboyHqx());
      await WasmBoy.config({});
      WasmBoy.setCanvas(canvasElement);

      await WasmBoy.loadROM(GameROMs.TobuTobuGirl);
      WasmBoy.play();
    };
    playROMTask();
  }

  render() {
    return (
      <div>
        <h1>wasmboy-plugin-hqx</h1>
        <h3>Demo Rom: <a href="http://tangramgames.dk/tobutobugirl/" target="_blank">Tobu Tobu Girl</a></h3>
        <h3>Press Space to Play/Pause</h3>
        <canvas></canvas>
      </div>
    );
  }
}

render(<PluginDemo />, document.getElementById('root'));


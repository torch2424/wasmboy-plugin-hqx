import { h, render, Component } from 'preact';

import {WasmBoy} from '../../wasmboy/dist/wasmboy.wasm.esm';

import './demo.css';

import wasmboyHqx from '../lib/lib';

// Import our open source roms
import { GameROMs } from 'shared-gb/openSourceROMs/urlImports';

console.log('wasmboy-plugin-hqx demo');
console.log('WasmBoy', WasmBoy);

class PluginDemo extends Component {
  componentDidMount() {
    // Get our HTML5 Canvas element
    const canvasElement = document.querySelector('canvas');

    const playROMTask = async () => {
      WasmBoy.addPlugin(wasmboyHqx());
      await WasmBoy.config({});
      WasmBoy.setCanvas(canvasElement);

      await WasmBoy.loadROM(GameROMs.TobuTobuGirl);
      console.log('Ready');
    };
    playROMTask();
  }

  render() {
    return (
      <div>
        <h1>wasmboy-plugin-hqx</h1>
        <h3>Demo Rom: <a href="http://tangramgames.dk/tobutobugirl/" target="_blank">Tobu Tobu Girl</a></h3>
        <div>
          <button onClick={() => WasmBoy.play()}>Play</button>
          <button onClick={() => WasmBoy.pause()}>Pause</button>
        </div>
        <br />
        <canvas></canvas>
      </div>
    );
  }
}

render(<PluginDemo />, document.getElementById('root'));


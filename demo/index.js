import { h, render, Component } from 'preact';

import wasmboyHqx from '../lib/index';

const init = () => {
  console.log('wasmboy-plugin-hqx demo');
  wasmboyHqx();
};
init();

class PluginDemo extends Component {
  componentDidMount() {}

  render() {
    return (
      <div>
        <h1>wasmboy-plugin-hqx</h1>
        <canvas id="wasmboy-canvas"></canvas>
      </div>
    );
  }
}

render(<PluginDemo />, document.getElementById('root'));


import resolve from 'rollup-plugin-node-resolve';
import commonjs from 'rollup-plugin-commonjs';
import babel from 'rollup-plugin-babel';
import serve from 'rollup-plugin-serve';
import bundleSize from 'rollup-plugin-bundle-size';
import postcss from 'rollup-plugin-postcss';
import postcssImport from 'postcss-import';
import compiler from '@ampproject/rollup-plugin-closure-compiler';
import json from 'rollup-plugin-json';
import pkg from './package.json';

const fs = require('fs');

const writeIndexHtmlToBuild = bundleName => {
  let indexHtml = fs.readFileSync('demo/index.html', 'utf8');
  indexHtml = indexHtml.replace('<%BUNDLE%>', bundleName.replace('build/', ''));
  if (!fs.existsSync('build/')){
    fs.mkdirSync('build/');
  }
  fs.writeFileSync('build/index.html', indexHtml, 'utf8');
};

const babelPluginConfig = {
  exclude: ['node_modules/**'],
  plugins: [
    ['@babel/plugin-proposal-class-properties'],
    ['@babel/plugin-proposal-object-rest-spread'],
    ['@babel/plugin-transform-react-jsx', { pragma: 'h' }],
    ['@babel/plugin-proposal-export-default-from']
  ]
};

let plugins = [
  json({
    exclude: 'node_modules/**'
  }),
  postcss({
    extensions: ['.css'],
    plugins: [postcssImport()]
  }),
  babel(babelPluginConfig),
  resolve(),
  commonjs()
];

if (process.env.ROLLUP_WATCH) {
  plugins = [
    ...plugins,
    serve({
      contentBase: ['dist/', 'build/', 'lib/'],
      port: 8080
    })
  ]
}

writeIndexHtmlToBuild('index.iife.js');

export default [
	{
    input: 'lib/index.js',
		output: [
      { 
        file: pkg.main, 
        format: 'cjs' 
      },
      { 
        file: pkg.module, 
        format: 'es' 
      }
		],
    plugins : [
      ...plugins,
      compiler(),
      bundleSize()
    ]
  },
  {
    input: 'lib/index.js',
    output: [
      { 
        file: pkg.main, 
        format: 'cjs' 
      }
    ],
    plugins
  },
  {
    input: 'demo/index.js',
    output: [
      { file: 'build/index.iife.js', format: 'iife' }
    ],
    plugins
  }
];

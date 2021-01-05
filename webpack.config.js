const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

const dist = path.resolve(__dirname, 'dist')

module.exports = {
  mode: 'production',
  entry: path.resolve(__dirname, 'js', 'index.js'),
  output: {
    filename: 'bundle.js',
    path: dist
  },
  devtool: 'source-map',
  devServer: {
    contentBase: dist
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'static', 'index.html')
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, 'rust'),
      outDir: path.resolve(__dirname, 'wasm')
    })
  ],
  experiments: {
    syncWebAssembly: true
  }
}

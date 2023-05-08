const HtmlWebpackPlugin = require('html-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const CopyWebpackPlugin = require('copy-webpack-plugin')

module.exports = {
	mode: 'production',
	entry: __dirname + '/js/index.js',
	output: {
		filename: 'bundle.js',
		path: __dirname + '/dist',
	},
	devtool: 'source-map',
	devServer: {
		contentBase: __dirname + '/dist',
	},
	plugins: [
		new HtmlWebpackPlugin({
			template: __dirname + '/index.html',
		}),
		new WasmPackPlugin({
			crateDirectory: __dirname + '/rust',
			outDir: __dirname + '/wasm',
		}),
		new CopyWebpackPlugin({
			patterns: [{ from: __dirname + '/static/' }],
		}),
	],
	module: {
		rules: [
			{
				test: /\.png$/i,
				type: 'asset/resource',
			},
		],
	},
	experiments: {
		syncWebAssembly: true,
	},
}

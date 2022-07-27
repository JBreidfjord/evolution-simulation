import * as path from 'path';

import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import * as webpack from 'webpack';

const config: webpack.Configuration = {
	entry: './src/index.tsx',
	output: {
		path: path.resolve(__dirname, 'build'),
		filename: 'bundle.js',
	},
	module: {
		rules: [
			{
				test: /\.tsx?$/,
				use: 'ts-loader',
				exclude: /node_modules/,
			},
			{
				test: /\.(js|jsx)$/,
				exclude: /node_modules/,
				use: {
					loader: 'babel-loader',
				},
			},
			{
				test: /\.css$/,
				use: ['style-loader', 'css-loader'],
			},
			{
				test: /\.(woff(2)?|ttf|otf|eot)$/,
				type: 'asset/resource',
				generator: {
					filename: 'fonts/[name][ext]',
				},
			},
		],
	},
	resolve: {
		extensions: ['.tsx', '.ts', '.js'],
	},
	plugins: [
		new HtmlWebpackPlugin({
			template: path.resolve('./public/index.html'),
			filename: 'index.html',
		}),
		new WasmPackPlugin({
			crateDirectory: path.resolve(__dirname, './libs/simulation-wasm/'),
			watchDirectories: [path.resolve(__dirname, './libs/')],
			outDir: path.resolve(__dirname, 'wasm'),
			outName: 'simulation',
		}),
	],
	mode: 'development',
	devtool: 'inline-source-map',
	experiments: {
		asyncWebAssembly: true,
	},
	ignoreWarnings: [
		(warning) =>
			warning.message === 'Critical dependency: the request of a dependency is an expression',
	],
};

export default config;
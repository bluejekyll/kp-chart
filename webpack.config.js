const path = require('path');
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
    entry: './js/index.js',
    output: {
        filename: 'kp-chart.js',
        path: path.resolve(__dirname, 'dist'),
    },
    stats: "errors-only",
    plugins: [
        new CopyPlugin([
            {
                from: path.resolve(__dirname, "static"),
                to: path.resolve(__dirname, 'dist')
            }
        ],
            { logLevel: 'warn' }
        ),
        new WasmPackPlugin({
            crateDirectory: __dirname, // Define where the root of the rust code is located (where the cargo.toml file is located)
        }),
    ]
};
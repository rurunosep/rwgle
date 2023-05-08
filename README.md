# Rwgle (Rust WebGL Engine)

Simple 3D rendering engine in the browser. Written in Rust compiled to WebAssembly and using WebGL. Read "roogle".

The Rust+WASM+JS app boilerplate is based on my [rust-wasm-template](https://github.com/rurunosep/rust-wasm-template) using wasm-pack and Webpack; most of the tools and libraries used are described there.

This was my first time using Rust, and first time compiling to WASM, and first time using JS across a language boundary, and first time using WebGL, and first time writing 3D shaders, and first time getting all those moving parts and more together with Webpack. Needless to say, it was challenging, but also very fun. I look forward to getting back to this soon with far more Rust experience.

## Features

- 🖼️ Basic texturing
- ↗️ Normal mapping for the bumpy bits
- 💎 Specular mapping for the shiny bits
- ☀️ Multiple directional and point lights
- 📦 (Extremely limited) glTF loading
- 🧵 Asynchronous resource loading

## Building and Running

Requires Rust and wasm-pack installed.

```
npm install
npm run build
npm run start
```

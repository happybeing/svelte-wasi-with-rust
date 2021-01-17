# Svelte Rust/WASI in Browser Example

This example demonstrates running Rust/Wasm compiled for WASI (the Web Assembly System Interface) in the browser using [WasmerJS]("https://github.com/wasmerio/wasmer-js).

For a non-Rust example and Svelte + Wasmer/WASI template see [simple-svelte-wasmer-webpack]{https://github.com/happybeing/simple-svelte-wasmer-webpack} which was used as the starting point for this project.

## Features
- [x] A Svelte WASI app with Rust subsystem
- [x] Rust and JS can access same wasmFS filesystem (temp disabled in Rust)
- [x] JS can call Rust (no params)
- [x] Rust can call JS (no params)
- [ ] Add passing params, return values using wasm-bindgen

# Instructions

## Prerequisites
You need Node.js and of course Rust.

## Build
If you don't have `yarn` use `npm run` instead of `yarn` in the following:
```bash
git clone https://github.com/happybeing/svelte-wasi-with-rust
cd svelte-wasi-with-rust
yarn && yarn dev-wasm-bindgen && yarn-dev
```
Once the build completes you can visit the app at localhost:8080.

# Development

The App code is in `src/App.svelte` and the Rust subsystem is in `src/rust-wasi-example`.

## Watching builds
In Linux, you can use `inotify` to re-build the app on changes to the Rust subsystem as follows.
In one terminal watch and re-build the app with:
```bash
yarn dev
```
In a second terminal watch and re-build the Rust subsystem with:
```bash
yarn watch-wasm-bindgen
```

If you're using VSCode, we recommend installing the offical Svelte extension as well as the offical Rust extension. If you are using other editors, your may need to install a plugin in order to get syntax highlighting and intellisense for both Svelte and Rust.


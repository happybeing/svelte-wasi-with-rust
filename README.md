# Svelte Rust/WASI in Browser Example

**Status:** WORK IN PROGRESS - this probably doesn't do anything yet

This example demonstrates running Rust/Wasm compiled for WASI (the Web Assembly System Interface) in the browser using [WasmerJS]("https://github.com/wasmerio/wasmer-js).

For a non-Rust example and Svelte + Wasmer/WASI template see [simple-svelte-wasmer-webpack]{https://github.com/happybeing/simple-svelte-wasmer-webpack} which was used as the starting point for this project.
# Instructions

## Prerequisites
You need Node.js and of course Rust.

## Build
If you don't have `yarn` use `npm run` instead of `yarn` in the following:
```
git clone https://github.com/happybeing/svelte-wasi-with-rust
cd svelte-wasi-with-rust
yarn && yarn dev
```
Once the build completes you can visit the app at localhost:8080.

# Development

The main code is in `src/App.svelte`.

If you're using VSCode, we recommend installing the offical Svelte extension as well as the offical Rust extension. If you are using other editors, your may need to install a plugin in order to get syntax highlighting and intellisense for both Svelte and Rust.


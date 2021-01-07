<script>
import {onMount} from "svelte"

import { WASI } from '@wasmer/wasi'
import browserBindings from '@wasmer/wasi/lib/bindings/browser'
import { WasmFs } from '@wasmer/wasmfs'

const wasmFilePath = '/helloworld.wasm'  // Path to our WASI module
const echoStr      = 'Hello World!'    // Text string to echo

onMount(() => {

	// Instantiate new WASI and WasmFs Instances
	// IMPORTANT:
	// Instantiating WasmFs is only needed when running in a browser.
	// When running on the server, NodeJS's native FS module is assigned by default
	const wasmFs = new WasmFs()

	let wasi = new WASI({
	// Arguments passed to the Wasm Module
	// The first argument is usually the filepath to the executable WASI module
	// we want to run.
	args: [wasmFilePath, echoStr],

	// Environment variables that are accesible to the WASI module
	env: {},

	// Bindings that are used by the WASI Instance (fs, path, etc...)
	bindings: {
		...browserBindings,
		fs: wasmFs.fs
	}
	})

	// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
	// Async function to run our WASI module/instance
	const startWasiTask =
	async pathToWasmFile => {
		// Fetch our Wasm File
		let response  = await fetch(pathToWasmFile)
		let wasmBytes = new Uint8Array(await response.arrayBuffer())

		// IMPORTANT:
		// Some WASI module interfaces use datatypes that cannot yet be transferred
		// between environments (for example, you can't yet send a JavaScript BigInt
		// to a WebAssembly i64).  Therefore, the interface to such modules has to
		// be transformed using `@wasmer/wasm-transformer`, which we will cover in
		// a later example

		// Instantiate the WebAssembly file
		let wasmModule = await WebAssembly.compile(wasmBytes);
		let instance = await WebAssembly.instantiate(wasmModule, {
		...wasi.getImports(wasmModule)
		});

		wasi.start(instance)                      // Start the WASI instance
		let stdout = await wasmFs.getStdOut()     // Get the contents of stdout

		const containerElement = document.getElementById("#stdout");
		containerElement.innerText = `${stdout}`;
	}

	// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
	// Everything starts here
	startWasiTask(wasmFilePath)

});
	
// Later, when we are done with the terminal, destroy it
// wasmTerminal.destroy();
</script>

<main>
<h1>Svelte WasmerJS/WASI Example.</h1>

<p>This example uses Web Assembly compiled for WASI (the Web
Assembly System Interface) running in the browser using <a
href="https://github.com/wasmerio/wasmer-js">WasmerJS</a>. The source code is at <a
href="https://github.com/happybeing/svelte-wasi-with-rust">svelte-wasi-with-rust</a></p>

<p>To do: add a Rust/WASI component</p>
<h2>Content from WASI</h2>
<p>
	<b>Standard output:</b>
	<span id="#stdout" name="#root"></span> 
</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
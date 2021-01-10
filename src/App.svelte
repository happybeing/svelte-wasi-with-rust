<script>
import {onMount} from "svelte"

import { WASI } from '@wasmer/wasi'
import browserBindings from '@wasmer/wasi/lib/bindings/browser'
import { WasmFs } from '@wasmer/wasmfs'

// Example 1 from the Wasmer.io docs
// const wasmFilePath = '/helloworld.wasm'  // Prints string param to stdout (from Wasmer.io docs)
// const echoStr      = 'Hello World!'    // Text string to echo

// Example 2 calls the hQ9+ Rust example
const wasmFilePath = '/wasm/wasi-example.wasm'	// The hQ9+ Rust example
 

let output = "";

// Instantiate new WASI and WasmFs Instances
// IMPORTANT:
// Instantiating WasmFs is only needed when running in a browser.
// When running on the server, NodeJS's native FS module is assigned by default
const wasmFs = new WasmFs()

let wasi = new WASI({
	// Arguments passed to the Wasm Module
	// The first argument is usually the filepath to the executable WASI module
	// we want to run.

	// For example 1:
	// args: [wasmFilePath, echoStr],

	// For example 2:
	// args: [wasmFilePath, "-e 'HQ9+'"],	// Shows control as CLI parameter
	args: [wasmFilePath, "-fHQ9.txt"],	// Shows control from a file 

	// Environment variables that are accesible to the WASI module
	env: {},

	// Bindings that are used by the WASI Instance (fs, path, etc...)
	bindings: {
		...browserBindings,
		fs: wasmFs.fs
	},
	preopens: {'/': '/', '.': '.'},	// Necessary for the Rust app can access wasmFs (example 2)
})

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Async function to run our WASI module/instance
const startWasiTask = async (pathToWasmFile) => {
	// Fetch the Wasm File
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

	// Sync write is easy:
	// wasmFs.fs.writeFileSync("HQ9.txt", "HHHH+Q", 'utf8', callback);

	// Async write needs to be completed before calling the Rust app:
	//
	// The wasmFs.fs API is asynchronous using callbacks, so wrap in a
	// promise or use the callback to invoke the Rust app. If not
	// the wasm app won't see the file you've created.
	await new Promise( (resolve, reject) => {
		wasmFs.fs.open("HQ9.txt", 'w+', (err, fd) => {
			let buf = Buffer.from('H+H'),
			pos = 0, offset = 0,
			len = buf.length;
			wasmFs.fs.write(fd, buf, offset, len, pos, (err,bytes,buff) => {
				let buf2 = Buffer.alloc(len);
				wasmFs.fs.read(fd,buf2,offset, len, pos, (err,bytes,buff2) => {
					console.log(buff2.toString());
					return resolve();
				});
			});
		});
	});

	wasmFs.fs.readdir('/', (e, files) => {
		if (e) console.log("error:", e);
		if (files !== undefined) {
			console.dir(files)
		}
	});

	wasi.start(instance)                      // Start the WASI instance
	output = await wasmFs.getStdOut()

	await wasmFs.fs.readdir('/', (e, files) => {
		if (e) console.log("error:", e);
		if (files !== undefined) {
			console.dir(files)
		}
	});
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Everything starts here
startWasiTask(wasmFilePath)
</script>

<main>
<h1>Svelte WasmerJS/WASI Example.</h1>
	<p>This example uses Web Assembly compiled for WASI (the Web
	Assembly System Interface) running in the browser using <a
	href="https://github.com/wasmerio/wasmer-js">WasmerJS</a>. The source code is at <a
	href="https://github.com/happybeing/svelte-wasi-with-rust">svelte-wasi-with-rust</a></p>

	<p>To do: add a Rust/WASI component</p>
	<h2>Content from WASI</h2>
		<h3>Standard output:</h3>
		<p>{output}</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		text-align: center;
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
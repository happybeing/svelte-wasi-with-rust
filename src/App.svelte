<script>
export let greet;	// Rust wasm (build using wasm-pack, so not WASI)

import {onMount} from "svelte"

import WasmTerminal, { fetchCommandFromWAPM } from "@wasmer/wasm-terminal";
import { lowerI64Imports } from "@wasmer/wasm-transformer";

// Handler for the fetchCommand property of the WasmTerminal Config.
const fetchCommandHandler = async ({args}) => {
  let commandName = args[0];
  // Return a "CallbackCommand" if our command matches a special name
  if (commandName === "callback-command") {
    const callbackCommand = async (options, wasmFs) => {
      return `Callback Command Working! Options: ${options}, fs: ${wasmFs}`;
    };
    return callbackCommand;
  }

  // Fetch a wasm Binary from WAPM for the command name.
  const wasmBinary = await fetchCommandFromWAPM({args});

  // lower i64 imports from Wasi Modules, so that most Wasi modules
  // Can run in a Javascript context.
  return await lowerI64Imports(wasmBinary);
};

onMount(() => {
	// Create our Wasm Terminal
	const wasmTerminal = new WasmTerminal({
		// Function that is run whenever a command is fetched
		fetchCommand: fetchCommandHandler
	});
	
	// Print out our initial message
	wasmTerminal.print("Hello World!");
	
	// Bind our Wasm terminal to it's container
	const containerElement = document.getElementById("#root");
	wasmTerminal.open(containerElement);
	wasmTerminal.fit();
	wasmTerminal.focus();
});
	
// Later, when we are done with the terminal, destroy it
// wasmTerminal.destroy();
</script>

<main>
<h1>Svelte Rust/WASI Example.</h1>

<p>This example demonstrates running Rust/Wasm compiled for WASI (the Web
Assembly System Interface) in the browser using <a
href="https://github.com/wasmerio/wasmer-js">WasmerJS</a>.</p>

The project is based on the <a
href="https://github.com/happybeing/svelte-wasm-terminal-test">svelte-wasm-terminal-test</a>
which shows how to run WASI in the browser uing wasm-terminal from WasmerJS.
<div id="#root" name="#root"></div> </main>

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
import App from './App.svelte';
import wasm from './greeting/Cargo.toml';

const init = async() => {
	const greet = await wasm();

	const app = new App({
		target: document.body,
		props: {
			greet: greet.greet()
		}
	});
};

init();
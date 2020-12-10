build: trunk
	trunk build

trunk: rustup
	cargo install trunk
	cargo install wasm-bindgen-cli --version 0.2.69

rustup:
	rustup default stable
	rustup target add wasm32-unknown-unknown 

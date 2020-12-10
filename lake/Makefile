build: trunk
	trunk build

trunk: rustup
	cargo install trunk wasm-bindgen-cli

rustup:
	rustup default stable
	rustup target add wasm32-unknown-unknown 

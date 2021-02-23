SHELL := /bin/bash

dev: trunk
	trunk serve

build: trunk
	trunk build

trunk: rustup
	wget -qO- https://github.com/thedodd/trunk/releases/download/v0.8.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
	wget -qO- https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.69/wasm-bindgen-0.2.69-x86_64-unknown-linux-musl.tar.gz | tar -xzf-
	mv ./wasm-bindgen-0.2.69-x86_64-unknown-linux-musl/wasm-bindgen .
	PATH=.:${PATH};

rustup:
	rustup default stable
	rustup target add wasm32-unknown-unknown 

emerson-build:
	cargo build --release --bin emerson

emerson-dev:
	cargo run --bin emerson

lake-build:
	cd lake && make build

lake-dev:
	cd lake && make dev

test:
	cargo test
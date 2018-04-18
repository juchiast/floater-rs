build:
	cargo web build
	mkdir dist -p
	cp ./target/wasm32-unknown-unknown/release/floater.wasm dist
	cp ./target/wasm32-unknown-unknown/release/floater.js dist

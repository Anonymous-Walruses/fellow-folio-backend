.PHONY: build-debug
build-debug: 
	rustup override set nightly
	cargo build

.PHONY: run-debug
run-debug: build-debug
	./target/debug/fellowfolio-backend

.PHONY: build
build:
	rustup override set nightly
	cargo build --release

.PHONY: run
run: build
	./target/release/fellowfolio-backend


	
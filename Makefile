.PHONY: run
run:
	cargo run

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: release
release:
	cargo build --release

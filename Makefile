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

.PHONY: install
install:
	mv target/release/a /usr/bin/

.PHONY: publish
publish:
	cargo publish

.PHONY: fmt
fmt:
	rustfmt **/*.rs

.PHONY: lint
lint:
	cargo clippy

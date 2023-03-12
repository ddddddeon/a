FEATURES=clipboard

.PHONY: run
run:
	cargo run --features $(FEATURES)

.PHONY: build
build:
	cargo build --features $(FEATURES)

.PHONY: test
test:
	cargo test --features $(FEATURES)

.PHONY: release
release:
	cargo build --release --features $(FEATURES)

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

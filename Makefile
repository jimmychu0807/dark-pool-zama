.PHONY: check
check:
	cargo check && cargo clippy

.PHONY: check-remote
check-remote:
	cargo remote -r dev -- check && cargo remote -r dev -- clippy

.PHONY: build
build:
	cargo build -r

.PHONY: build-remote
build-remote:
	cargo remote -r dev -- build -r

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: example
example:
	cargo remote -r dev -- run -r --example plain_dark_pool

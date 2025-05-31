.PHONY: check
check:
	cargo check && cargo clippy

.PHONY: check-remote
check-remote:
	cargo remote -r dev -- check && cargo remote -r dev -- clippy

.PHONY: build
build:
	cargo build -r --all --examples

.PHONY: build-remote
build-remote:
	cargo remote -r dev -- build -r --all --examples

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: example-remote
example-remote:
	cargo remote -r dev -- run -r --example $(filter-out $@,$(MAKECMDGOALS))

.PHONY: test-remote
test-remote:
	cargo remote -r dev -- test -r -- --nocapture

%::
	@:

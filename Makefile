.PHONY: all fmt fmt-check lint test build bindings deploy clean

all: fmt-check lint test build

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint: fmt-check
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo test --workspace

build:
	./scripts/build.sh

bindings: build
	./scripts/generate-bindings.sh

# Usage: make deploy NETWORK=testnet
deploy: build
	./scripts/deploy.sh $(NETWORK)

clean:
	cargo clean

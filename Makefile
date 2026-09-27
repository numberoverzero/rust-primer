.DEFAULT_GOAL := all
.PHONY: all build test lint fmt fmt-check vet bench clean

all: build test lint fmt-check vet

build:
	cargo build --locked --all-targets

test:
	cargo test --locked

lint:
	cargo clippy --locked --all-targets -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

vet:
	@set -e; tree="$$(cargo tree --locked --offline --prefix none)"; \
	if [ "$$(printf '%s\n' "$$tree" | wc -l | tr -d ' ')" = 1 ]; then \
		printf '%s\n' 'No third-party dependencies to vet.'; \
	else \
		cargo vet --locked; \
	fi

bench:
	cargo bench --locked --bench bits

clean:
	cargo clean

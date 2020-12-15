# requires cargo-watch
watch:
	cargo watch -x clippy

check:
	cargo test

# run these in release mode because the reason they're ignored is usually that
# they take a long time to run
check-all: check
	cargo test --release -- --ignored

.PHONY: watch check check-all

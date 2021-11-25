# requires cargo-watch
watch:
	cargo watch -x clippy

check:
	cargo test --workspace --release

check-all:
	cargo test --workspace --release -- --include-ignored

.PHONY: watch check check-all

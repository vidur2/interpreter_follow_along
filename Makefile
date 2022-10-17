.PHONY: release
release:
	cargo build --release
	cp ./target/release/vmod /usr/local/bin
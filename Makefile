.PHONY: release
release:
	cargo build --release
	cp ./target/release/vmod /usr/local/bin
	cp ./target/release/vpm /usr/local/bin

.PHONY: debug
debug:
	cargo build
	cp ./target/debug/vmod /usr/local/bin
	cp ./target/debug/vpm /usr/local/bin
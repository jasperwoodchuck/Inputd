.PHONY: run sudo-run lint fmt clean edit

run:
	cargo run

sudo-run:
	cargo build
	sudo -E target/debug/inputd

lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo +nightly fmt

clean:
	cargo clean

edit:
	zed ~/.config/inputd/config.binds

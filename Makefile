.PHONY: build-release 

cargo_check:
	cargo check
	cargo check --no-default-features -F json
	cargo check --no-default-features -F sqlite

run_test:
	cargo test

clippy:
	cargo clippy

build-release:
	cargo build --release

release-mac: build-release
	strip target/release/tlogbook
	otool -L target/release/tlogbook
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/tlogbook-mac.tar.gz ./tlogbook
	ls -lisah ./release/tlogbook-mac.tar.gz

release-win: build-release
	mkdir -p release
	7z -y a ./release/tlogbook-win.zip ./target/release/tlogbook.exe

release-linux: 
	cargo build --release --target=x86_64-unknown-linux-gnu
	strip target/x86_64-unknown-linux-gnu/release/tlogbook
	mkdir -p release
	tar -C ./target/x86_64-unknown-linux-gnu/release/ -czvf ./release/tlogbook-linux-gnu.tar.gz ./tlogbook

install:
	cargo install --path "." 

install_sqlite:
	cargo install --path "." --no-default-features -F sqlite 

install_json:
	cargo install --path "." --no-default-features -F json 


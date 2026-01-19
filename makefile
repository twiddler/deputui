$(info Making deputui and deputui-* binaries …)

build:
	cargo build --release --workspace

install:
	sudo cp target/release/deputui /usr/bin/deputui
	sudo cp target/release/deputui-pnpm /usr/bin/deputui-pnpm
	sudo cp target/release/deputui-review /usr/bin/deputui-review

all: build install


name := outdatui
helper := outdat-list
$(info Making ${name} …)
$(info Making ${helper} …)

build:
	cargo build --release --workspace

install:
	sudo cp target/release/$(name) /usr/bin/$(name)
	sudo cp target/release/$(helper) /usr/bin/$(helper)

all: build install


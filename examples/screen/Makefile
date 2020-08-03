.PHONY: all clean

all:
	docker run --rm -v $(PWD)/../../:/ev3dev-lang-rust/ -w  /ev3dev-lang-rust/examples/screen pixix4/ev3dev-rust-cross cargo build --release --target armv5te-unknown-linux-gnueabi

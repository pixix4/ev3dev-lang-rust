.PHONY: all clean

all:
	docker run --rm -v $(PWD)/../../:/ev3dev-lang-rust/ -w /ev3dev-lang-rust/examples/attributes pixix4/ev3dev-rust cargo build --release --target armv5te-unknown-linux-gnueabi

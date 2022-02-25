all: build

build:
	docker run --rm -v $(PWD):/build -w /build pixix4/ev3dev-rust:latest \
		cargo build --release --examples --target armv5te-unknown-linux-musleabi

all: build strip

build:
	docker run --rm -v $(PWD):/build -w /build pixix4/ev3dev-rust:latest \
		cargo build --release --examples --target armv5te-unknown-linux-musleabi

strip: strip-buttons strip-color-sensor strip-custom-attributes strip-infrared-sensor strip-motors

strip-buttons:
	docker run --rm -v $(PWD):/build -w /build pixix4/ev3dev-rust:latest \
		arm-linux-gnueabi-strip /build/target/armv5te-unknown-linux-musleabi/release/examples/buttons
strip-color-sensor:
	docker run --rm -v $(PWD):/build -w /build pixix4/ev3dev-rust:latest \
		arm-linux-gnueabi-strip /build/target/armv5te-unknown-linux-musleabi/release/examples/color-sensor
strip-custom-attributes:
	docker run --rm -v $(PWD):/build -w /build pixix4/ev3dev-rust:latest \
		arm-linux-gnueabi-strip /build/target/armv5te-unknown-linux-musleabi/release/examples/custom-attributes
strip-infrared-sensor:
	docker run --rm -v $(PWD):/build -w /build pixix4/ev3dev-rust:latest \
		arm-linux-gnueabi-strip /build/target/armv5te-unknown-linux-musleabi/release/examples/infrared-sensor
strip-motors:
	docker run --rm -v $(PWD):/build -w /build pixix4/ev3dev-rust:latest \
		arm-linux-gnueabi-strip /build/target/armv5te-unknown-linux-musleabi/release/examples/motors

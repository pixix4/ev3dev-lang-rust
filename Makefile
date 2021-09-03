build-example-attributes:
	docker run --rm -v $(PWD):/ev3dev-lang-rust/ -w /ev3dev-lang-rust/examples/attributes pixix4/ev3dev-rust /bin/bash -c "cargo build --release --target armv5te-unknown-linux-gnueabi && /usr/bin/arm-linux-gnueabi-strip /ev3dev-lang-rust/target/armv5te-unknown-linux-gnueabi/release/attributes"

build-example-buttons:
	docker run --rm -v $(PWD):/ev3dev-lang-rust/ -w /ev3dev-lang-rust/examples/buttons pixix4/ev3dev-rust /bin/bash -c "cargo build --release --target armv5te-unknown-linux-gnueabi && /usr/bin/arm-linux-gnueabi-strip /ev3dev-lang-rust/target/armv5te-unknown-linux-gnueabi/release/buttons"

build-example-color-sensor:
	docker run --rm -v $(PWD):/ev3dev-lang-rust/ -w /ev3dev-lang-rust/examples/color-sensor pixix4/ev3dev-rust /bin/bash -c "cargo build --release --target armv5te-unknown-linux-gnueabi && /usr/bin/arm-linux-gnueabi-strip /ev3dev-lang-rust/target/armv5te-unknown-linux-gnueabi/release/color-sensor"

build-example-infrared-sensor:
	docker run --rm -v $(PWD):/ev3dev-lang-rust/ -w /ev3dev-lang-rust/examples/infrared-sensor pixix4/ev3dev-rust /bin/bash -c "cargo build --release --target armv5te-unknown-linux-gnueabi && /usr/bin/arm-linux-gnueabi-strip /ev3dev-lang-rust/target/armv5te-unknown-linux-gnueabi/release/infrared-sensor"

build-example-screen:
	docker run --rm -v $(PWD):/ev3dev-lang-rust/ -w /ev3dev-lang-rust/examples/screen pixix4/ev3dev-rust /bin/bash -c "cargo build --release --target armv5te-unknown-linux-gnueabi && /usr/bin/arm-linux-gnueabi-strip /ev3dev-lang-rust/target/armv5te-unknown-linux-gnueabi/release/screen"

build-examples: build-example-attributes build-example-buttons build-example-color-sensor build-example-infrared-sensor build-example-screen

clean:
	cargo clean

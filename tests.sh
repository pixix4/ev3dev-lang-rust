#!/bin/bash

cargo test
cargo test --package ev3dev-lang-rust --test ev3 --no-default-features --features ev3
cargo test --package ev3dev-lang-rust --test brickpi --no-default-features --features brickpi
cargo test --package ev3dev-lang-rust --test brickpi3 --no-default-features --features brickpi3
EV3DEV_DRIVER_PATH="/test/path" cargo test --package ev3dev-lang-rust --test override-driver-path --features override-driver-path

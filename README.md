# Rust language bindings for ev3dev

![Build](https://github.com/pixix4/ev3dev-lang-rust/workflows/Build/badge.svg)
[![Latest version](https://img.shields.io/crates/v/ev3dev-lang-rust.svg)](https://crates.io/crates/ev3dev-lang-rust)

## Notice

Currently this project is not compatible with the BrickPi platform.

## Usage

```rust
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::ColorSensor;

fn main() -> Ev3Result<()> {

    // Get large motor on port outA.
    let large_motor = LargeMotor::get(MotorPort::OutA)?;

    // Set command "run-direct".
    large_motor.run_direct()?;

    // Run motor.
    large_motor.set_duty_cycle_sp(50)?;

    // Find color sensor. Always returns the first recognised one.
    let color_sensor = ColorSensor::find()?;

    // Switch to rgb mode.
    color_sensor.set_mode_rgb_raw()?;

    // Get current rgb color tuple.
    println!("Current rgb color: {:?}", color_sensor.get_rgb()?);

    Ok(())
}
```

## Cross compile for ev3 brick

1. Create target configuration `.cargo/config`
    ```toml
    [target.armv5te-unknown-linux-gnueabi]
    linker = "/usr/bin/arm-linux-gnueabi-gcc"
    ```

2. Create Dockerfile
    ```dockerfile
    FROM debian:stretch

    RUN dpkg --add-architecture armel
    RUN apt update

    # Fix debian package alias
    RUN sed -i "s#deb http://security.debian.org/debian-security stretch/updates main#deb http://deb.debian.org/debian-security stretch/updates main#g" /etc/apt/sources.list

    # Install curl for rust installation
    # Install g++ as buildscript compiler
    # Install g++-arm-linux-gnueabi as cross compiler
    RUN apt --yes install curl g++ g++-arm-linux-gnueabi crossbuild-essential-armel

    # Instull rust for host platform
    RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    ENV PATH "$PATH:/root/.cargo/bin"

    # Add stdlib for target platform
    RUN rustup target add armv5te-unknown-linux-gnueabi

    # docker run -it --rm -v $PWD:/build/ -w /build pixix4/ev3dev-rust-cross
    # cargo build --release --target armv5te-unknown-linux-gnueabi
    ```

3. Build docker image
    ```bash
    docker build . -t pixix4/ev3dev-rust-cross --no-cache
    ```

4. Start docker image
    ```bash
    docker run -it --rm -v $PWD:/build/ -w /build pixix4/ev3dev-rust-cross
    ```

5. Build binary for ev3dev
    ```bash
    cargo build --release --target armv5te-unknown-linux-gnueabi
    ```
    The `--release` flag is optional. However, it can speedup the execution time by a factor of 30.

The target binary is now in `target/armv5te-unknown-linux-gnueabi/release/{application_name}`

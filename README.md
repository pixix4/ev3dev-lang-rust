# Rust language bindings for ev3dev

[![Build Status](https://travis-ci.org/pixix4/ev3dev-lang-rust.svg?branch=master)](https://travis-ci.org/pixix4/ev3dev-lang-rust)

## Notice

Currently this project is not compatible with the BrickPi platform. 

## Usage

```rust
extern crate ev3dev_lang_rust;

use std::io::Result;
use ev3dev_lang_rust::tacho_motor::{LargeMotor, TachoMotor};
use ev3dev_lang_rust::core::MotorPort;
use ev3dev_lang_rust::color_sensor::ColorSensor;

fn main() -> Result<()> {

    // Get large motor on port outA.
    let mut large_motor = LargeMotor::new(MotorPort::OutA).unwrap();

    // Set command "run-direct".
    large_motor.run_direct()?;

    // Run motor.
    large_motor.set_duty_cycle_sp(50)?;

    // Find color sensor. Always returns the first recognised one.
    let mut color_sensor = ColorSensor::find().unwrap();

    // Switch to rgb mode.
    color_sensor.set_mode_rgb_raw()?;

    // Get current rgb color tuple.
    println!("Current rgb color: {:?}", color_sensor.get_rgb()?);

    Ok(())
}
```

## Cross compile for ev3 brick

1. Create Dockerfile
    ```dockerfile
    FROM ubuntu:18.04
    
    RUN apt update
    
    # Install curl for rust installation
    # Install g++ as buildscript compiler
    # Install g++-arm-linux-gnueabi as cross compiler
    RUN apt --yes install curl g++ g++-arm-linux-gnueabi
    
    # Instull rust for host platform
    RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
    
    ENV PATH "$PATH:/root/.cargo/bin"
    
    # Add stdlib for target platform
    RUN rustup target add armv5te-unknown-linux-gnueabi
    ```

2. Build docker image
    ```bash
    docker build . -t pixix4/ev3dev-rust-cross
    ```

3. Start docker image
    ```bash
    docker run -it --rm -v $PWD:/build/ -w /build pixix4/ev3dev-rust-cross
    ```

4. Build binary for ev3dev
    ```bash
    cargo build --release --target armv5te-unknown-linux-gnueabi
    ```
    The `--release` flag is optional. However, it can speedup the execution time by a factor of 30.
    
The target binary is now in `target/armv5te-unknown-linux-gnueabi/release/libev3dev_lang_rust.rlib`

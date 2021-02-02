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

## Cross compilation for the ev3 robot

1. Create target configuration in `.cargo/config`
    ```toml
    [target.armv5te-unknown-linux-gnueabi]
    linker = "/usr/bin/arm-linux-gnueabi-gcc"
    ```

2. Get the docker image. You can either download the prebuild image or build it yourself with the provided Dockerfile (`docker/Dockerfile`).
    ```bash
    docker pull pixix4/ev3dev-rust

    # or

    docker build . -t pixix4/ev3dev-rust --no-cache
    ```

3. Build binary
    ```bash
    # Run in interactive docker shell
    docker run -it --rm -v $PWD:/build/ -w /build pixix4/ev3dev-rust
    cargo build --release --target armv5te-unknown-linux-gnueabi

    # Run directly (e.g. via Makefile)
    docker run --rm -v $PWD:/build/ -w /build pixix4/ev3dev-rust \
            cargo build --release --target armv5te-unknown-linux-gnueabi
    ```
    The `--release` flag is optional. However, it can speed up the execution time by a factor of 30.

    The target binary is now in `target/armv5te-unknown-linux-gnueabi/release/{application_name}`

    If you use the direct method you will notice that each build gets stuck at `Updating crates.io index` for a long time. To speed up this step you can use the vendoring machanic of cargo.
    ```bash
    cargo vendor
    ```
    Execute the above command and add this addtional config to `.cargo/config`.
    ```toml
    [source.crates-io]
    replace-with = "vendored-sources"

    [source.vendored-sources]
    directory = "vendor"
    ```

## Editor support

If you have problems with code completion or inline documentation with rust analyzer it may help to enable to following settings:

```json
{
    "rust-analyzer.cargo.loadOutDirsFromCheck": true,
    "rust-analyzer.procMacro.enable": true,
}
```
(Example from VSCode `settings.json`)

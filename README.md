# Rust language bindings for ev3dev

![Build](https://github.com/pixix4/ev3dev-lang-rust/workflows/Build/badge.svg)
[![Latest version](https://img.shields.io/crates/v/ev3dev-lang-rust.svg)](https://crates.io/crates/ev3dev-lang-rust)

## Notice

To use this project with the BrickPi platform the corresponding feature has to be enabled. The features `ev3`, `brickpi` and `brickpi3` are mutual exclusive.
```toml
[dependencies]
ev3dev_lang_rust = { version="0.13.0" default-features=false, features=["brickpi"] }
```

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

    // Find color sensor. Always returns the first recognized one.
    let color_sensor = ColorSensor::find()?;

    // Switch to rgb mode.
    color_sensor.set_mode_rgb_raw()?;

    // Get current rgb color tuple.
    println!("Current rgb color: {:?}", color_sensor.get_rgb()?);

    Ok(())
}
```

There is a [template repository](https://github.com/pixix4/ev3dev-lang-rust-template/) that contains all the required configurations for cross-compilation and performance/binary-size optimizations for this "Hello World" example.

## Supported features

- Motors:
  - `LargeMotor` [`lego-ev3-l-motor`, `lego-nxt-motor`]
  - `MediumMotor` [`lego-ev3-m-motor`]
  - `TachoMotor`: Useful wrapper around `LargeMotor` and `MediumMotor` to make common functions easier to use
- Sensors:
  - `ColorSensor` [`lego-ev3-color`]
  - `CompassSensor` [`ht-nxt-compass`]
  - `GyroSensor` [`lego-ev3-gyro`]
  - `InfraredSensor` [`lego-ev3-ir`]
  - `IrSeekerSensor` [`ht-nxt-ir-seek-v2`]
  - `LightSensor` [`lego-nxt-light`]
  - `TouchSensor` [`lego-ev3-touch`, `lego-nxt-touch`]
  - `UltrasonicSensor` [`lego-ev3-us`, `lego-nxt-us`]
- Utility
  - `Button`: Provides access to the integrated buttons on the ev3 brick
  - `Led`: Provides access to the integrated led's on the ev3 brick
  - `PowerSupply`: Provides access to the power supply information
  - `Screen`: Provides access to the integrated display of the ev3 brick
  - `sound`: Provides access to the integrated speakers of the ev3 brick

## Cross compilation for the ev3 robot - using `musl` toolchain

1. Install the `armv5te-musl` toolchain

   ```bash
   rustup target add armv5te-unknown-linux-musleabi
   ```

2. Create `.cargo/config.toml` with the following content

   ```toml
   [build]
   target = "armv5te-unknown-linux-musleabi"

   [target.armv5te-unknown-linux-musleabi]
   linker = "rust-lld"
   ```

3. Build binary

   ```bash
   cargo build --release
   ```

   The `--release` flag is optional. However, it can speed up the execution time by a factor of 30.
   The target binary is now in `target/armv5te-unknown-linux-musleabi/release/{application_name}`.

## Cross compilation for the ev3 robot - using docker

If you need to cross compile other dependencies (eg. `openssl` or `paho-mqtt`) it is much easier to use a complete cross compile toolchain. For this you can use the provided docker image `pixix4/ev3dev-rust:latest`.

1. Setup a docker environment

2. Create `.cargo/config.toml` with the following content

   ```toml
   [build]
   target = "armv5te-unknown-linux-gnueabi"

   [target.armv5te-unknown-linux-gnueabi]
   linker = "/usr/bin/arm-linux-gnueabi-gcc"
   ```

3. Build binary

   ```bash
   docker run --rm -it -v $(pwd):/build -w /build pixix4/ev3dev-rust:latest \
      cargo build --release
   ```

   The `--release` flag is optional. However, it can speed up the execution time by a factor of 30.
   The target binary is now in `target/armv5te-unknown-linux-gnueabi/release/{application_name}`.

   If you do this you will notice that each build gets stuck at `Updating crates.io index` for a long time. To speed up this step you can use the vendoring mechanic of cargo.

   ```bash
   cargo vendor
   ```

   Execute the above command and add this additional config to `.cargo/config`.

   ```toml
   [source.crates-io]
   replace-with = "vendored-sources"

   [source.vendored-sources]
   directory = "vendor"
   ```

## Optimize binary size

- Enable "fat" link time optimizations and strip debug symbols:
  By default rust only performs lto for each crate individually. To enable global lto (which result in a much more aggressive dead code elimination) add the following additional config to your `Cargo.toml`. This also removes additional debug symbols from the binary. With this you can reduce the binary size of the "Hello World" example by more than 90%.

  ```toml
  [profile.release]
  lto = true
  strip = "debuginfo"
  ```

  The strip option requires rust `1.59.0`. If you are using an older version you can do this manually with docker:

  ```bash
  # Run in interactive docker shell
  docker run -it --rm -v $(PWD):/build/ -w /build pixix4/ev3dev-rust
  /usr/bin/arm-linux-gnueabi-strip /build/target/armv5te-unknown-linux-gnueabi/release/{application_name}

  # Run directly (e.g. via Makefile)
  docker run --rm -v $(PWD):/build/ -w /build pixix4/ev3dev-rust \
       /usr/bin/arm-linux-gnueabi-strip /build/target/armv5te-unknown-linux-gnueabi/release/{application_name}
  ```

## Editor support

If you have problems with code completion or inline documentation with rust analyzer it may help to enable to following settings:

```json
{
  "rust-analyzer.cargo.loadOutDirsFromCheck": true,
  "rust-analyzer.procMacro.enable": true
}
```

(Example from VSCode `settings.json`)

## Docs.rs documentation

To build the complete documentation (including the `screen` feature) use:

```bash
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --features ev3,screen
```

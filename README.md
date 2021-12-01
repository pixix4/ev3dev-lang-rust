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

There is a [template repository](https://github.com/pixix4/ev3dev-lang-rust-template/) that contains all the required configurations for cross-compilation and perfomance/binary-size optimizations for this "Hello World" example.

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
  - `Ev3Button`: Provides access to the integrated buttons on the ev3 brick
  - `Led`: Provides access to the integrated leds on the ev3 brick
  - `PowerSupply`: Provides access to the power supply information
  - `Screen`: Provides access to the integrated display of the ev3 brick
  - `sound`: Provides access to the integrated speakers of the ev3 brick

## Cross compilation for the ev3 robot

1. Install [`cross`](https://github.com/rust-embedded/cross) and the `armv5te` toolchain

   ```bash
   cargo install cross
   rustup target add armv5te-unknown-linux-gnueabi
   ```

2. Build binary with docker

   ```bash
   cross build --release --target armv5te-unknown-linux-gnueabi
   ```

   The `--release` flag is optional. However, it can speed up the execution time by a factor of 30.

   The target binary is now in `target/armv5te-unknown-linux-gnueabi/release/{application_name}`

## Alternative cross compilation for the ev3 robot

If the above compilation with `cross` failed you can try this manual approach.

1. Create target configuration in [`.cargo/config`](https://github.com/pixix4/ev3dev-lang-rust/blob/master/.cargo/config)

   ```toml
   [target.armv5te-unknown-linux-gnueabi]
   linker = "/usr/bin/arm-linux-gnueabi-gcc"
   ```

2. Get the docker image. You can either download the prebuild image or build it yourself with the provided Dockerfile ([`docker/Dockerfile`](https://github.com/pixix4/ev3dev-lang-rust/blob/master/docker/Dockerfile)).

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

## Optimize binary size

To reduce the resulting binary size you can try the following steps:

1. Enable "fat" link time optimizations
   By default rust only performs lto for each crate individually. To enable global lto (which result in a much more aggressive dead code elimination) add this addtional config to your `Cargo.toml`:

```toml
[profile.release]
lto = true
```

2. Strip debug symbols from the resulting binary
   Since the usage of an debugger is not really feasible you can strip (debug) symbols from the binary. To do this you

```bash
# Run in interactive docker shell
docker run -it --rm -v $PWD:/build/ -w /build pixix4/ev3dev-rust
/usr/bin/arm-linux-gnueabi-strip /build/target/armv5te-unknown-linux-gnueabi/release/{application_name}

# Run directly (e.g. via Makefile)
docker run --rm -v $PWD:/build/ -w /build pixix4/ev3dev-rust \
        /usr/bin/arm-linux-gnueabi-strip /build/target/armv5te-unknown-linux-gnueabi/release/{application_name}
```

With this you can reduce the binary size of the "Hello World" example by more than 90%.

## Editor support

If you have problems with code completion or inline documentation with rust analyzer it may help to enable to following settings:

```json
{
  "rust-analyzer.cargo.loadOutDirsFromCheck": true,
  "rust-analyzer.procMacro.enable": true
}
```

(Example from VSCode `settings.json`)

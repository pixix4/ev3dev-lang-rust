# Derive macros for ev3dev_lang_rust

[![Build Status](https://travis-ci.org/pixix4/ev3dev-lang-rust.svg?branch=master)](https://travis-ci.org/pixix4/ev3dev-lang-rust)
[![Latest version](https://img.shields.io/crates/v/ev3dev-lang-rust-derive.svg)](https://crates.io/crates/ev3dev-lang-rust-derive)

This crate provides some derive macros to simplify the codebase.

The following traits can be automatically derived:

* `Device`
* `Findable`
* `Motor`
* `TachoMotor`
* `ServoMotor`
* `DcMotor`
* `Sensor`

The findable derive needs 3 additional attributes.
* `class_name: &str`
* `driver_name: &str`
* `port: dyn ev3dev_lang_rust::Motor`

## Example

The functionallity of the `LargeMotor` struct consists complitly through derives:

```rust
#[derive(Debug, Clone, Device, Findable, Motor, TachoMotor)]
#[class_name = "tacho-motor"]
#[driver_name = "lego-ev3-l-motor"]
#[port = "crate::motors::MotorPort"]
pub struct LargeMotor {
    driver: Driver,
}
```
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

//! # Rust language bindings for ev3dev
//!
//! ```no_run
//! extern crate ev3dev_lang_rust;
//!
//! use ev3dev_lang_rust::Ev3Result;
//! use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
//! use ev3dev_lang_rust::sensors::ColorSensor;
//!
//! fn main() -> Ev3Result<()> {
//!
//!     // Get large motor on port outA.
//!     let large_motor = LargeMotor::get(MotorPort::OutA)?;
//!
//!     // Set command "run-direct".
//!     large_motor.run_direct()?;
//!
//!     // Run motor.
//!     large_motor.set_duty_cycle_sp(50)?;
//!
//!     // Find color sensor. Always returns the first recognized one.
//!     let color_sensor = ColorSensor::find()?;
//!
//!     // Switch to rgb mode.
//!     color_sensor.set_mode_rgb_raw()?;
//!
//!     // Get current rgb color tuple.
//!     println!("Current rgb color: {:?}", color_sensor.get_rgb()?);
//!
//!     Ok(())
//! }
//! ```

#[cfg(feature = "screen")]
extern crate framebuffer;

#[cfg(feature = "screen")]
extern crate image;

#[macro_use]
extern crate ev3dev_lang_rust_derive;
extern crate libc;

#[macro_use]
mod findable;

mod attribute;
pub use attribute::Attribute;
mod driver;
pub use driver::Driver;
#[cfg(feature = "override-driver-path")]
pub use driver::DRIVER_PATH;
mod device;
pub use device::Device;

mod utils;
pub use utils::{Ev3Error, Ev3Result, Port};

pub mod wait;

pub mod motors;
pub mod sensors;

#[cfg(feature = "ev3")]
mod ev3;
#[cfg(feature = "ev3")]
pub use ev3::Button;
#[cfg(feature = "ev3")]
pub use ev3::Led;
#[cfg(feature = "ev3")]
mod port_constants {
    pub const OUTPUT_A: &str = "outA";
    pub const OUTPUT_B: &str = "outB";
    pub const OUTPUT_C: &str = "outC";
    pub const OUTPUT_D: &str = "outD";

    pub const INPUT_1: &str = "in1";
    pub const INPUT_2: &str = "in2";
    pub const INPUT_3: &str = "in3";
    pub const INPUT_4: &str = "in4";
}

#[cfg(feature = "brickpi")]
mod brickpi;
#[cfg(feature = "brickpi")]
pub use brickpi::Led;
#[cfg(feature = "brickpi")]
mod port_constants {
    pub const OUTPUT_A: &str = "serial0-0:MA";
    pub const OUTPUT_B: &str = "serial0-0:MB";
    pub const OUTPUT_C: &str = "serial0-0:MC";
    pub const OUTPUT_D: &str = "serial0-0:MD";

    pub const INPUT_1: &str = "serial0-0:S1";
    pub const INPUT_2: &str = "serial0-0:S2";
    pub const INPUT_3: &str = "serial0-0:S3";
    pub const INPUT_4: &str = "serial0-0:S4";
}

#[cfg(feature = "brickpi3")]
mod brickpi3;
#[cfg(feature = "brickpi3")]
pub use brickpi3::Led;
#[cfg(feature = "brickpi3")]
mod port_constants {
    pub const OUTPUT_A: &str = "spi0.1:MA";
    pub const OUTPUT_B: &str = "spi0.1:MB";
    pub const OUTPUT_C: &str = "spi0.1:MC";
    pub const OUTPUT_D: &str = "spi0.1:MD";

    pub const INPUT_1: &str = "spi0.1:S1";
    pub const INPUT_2: &str = "spi0.1:S2";
    pub const INPUT_3: &str = "spi0.1:S3";
    pub const INPUT_4: &str = "spi0.1:S4";
}

pub mod sound;

mod power_supply;
pub use power_supply::PowerSupply;

#[cfg(feature = "screen")]
mod screen;
#[cfg(feature = "screen")]
pub use screen::Screen;

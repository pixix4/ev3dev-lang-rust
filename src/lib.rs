#![deny(missing_docs)]

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
//!     // Find color sensor. Always returns the first recognised one.
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

mod attriute;
pub use attriute::Attribute;
mod driver;
pub use driver::Driver;
mod device;
pub use device::Device;

mod utils;
pub use utils::{Ev3Error, Ev3Result, Port};

pub mod wait;

pub mod motors;
pub mod sensors;

mod led;
pub use led::Led;

pub mod sound;

mod buttons;
pub use buttons::Ev3Button;

mod power_supply;
pub use power_supply::PowerSupply;

#[cfg(feature = "screen")]
mod screen;
#[cfg(feature = "screen")]
pub use screen::Screen;

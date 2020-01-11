#![deny(missing_docs)]

//! # Rust language bindings for ev3dev
//!
//! ```no_run
//! extern crate ev3dev_lang_rust;
//!
//! use ev3dev_lang_rust::prelude::*;
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

#[macro_use]
extern crate ev3dev_lang_rust_derive;
extern crate libc;

mod attriute;
pub use attriute::Attribute;
mod driver;
pub use driver::Driver;
mod device;
pub use device::Device;
mod findable;
pub use findable::Findable;

mod utils;
pub use utils::{Ev3Error, Ev3Result, Port};

pub mod wait;

pub mod motors;
pub mod sensors;

pub mod led;
pub use led::Led;

pub mod sound;

mod power_supply;
pub use power_supply::PowerSupply;

pub mod prelude {
    //! The purpose of this module is to alleviate imports of many common ev3dev traits.
    //!
    //! ```
    //! use ev3dev_lang_rust::prelude::*;
    //! ```
    pub use motors::{DcMotor, Motor, ServoMotor, TachoMotor};
    pub use sensors::Sensor;
    pub use Device;
    pub use Findable;
}

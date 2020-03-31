//! # Container module for motor types

#[macro_use]
mod dc_motor_macro;
#[macro_use]
mod servo_motor_macro;
#[macro_use]
mod tacho_motor_macro;

mod large_motor;
pub use self::large_motor::LargeMotor;

mod medium_motor;
pub use self::medium_motor::MediumMotor;

use crate::Port;

/// EV3 ports `outA` to `outD`
#[derive(Debug, Copy, Clone)]
pub enum MotorPort {
    /// EV3 `outA` port
    OutA,
    /// EV3 `outB` port
    OutB,
    /// EV3 `outC` port
    OutC,
    /// EV3 `outD` port
    OutD,
}

impl Port for MotorPort {
    fn address(&self) -> String {
        match self {
            MotorPort::OutA => "outA".to_owned(),
            MotorPort::OutB => "outB".to_owned(),
            MotorPort::OutC => "outC".to_owned(),
            MotorPort::OutD => "outD".to_owned(),
        }
    }
}

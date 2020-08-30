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

impl MotorPort {
    /// Try to format a device name path to a  port name.
    pub fn format_name(name: &str) -> String {
        match name {
            "motor0" => MotorPort::OutA.address(),
            "motor1" => MotorPort::OutB.address(),
            "motor2" => MotorPort::OutC.address(),
            "motor3" => MotorPort::OutD.address(),
            _ => name.to_owned(),
        }
    }
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

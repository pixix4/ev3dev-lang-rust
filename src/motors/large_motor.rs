use super::MotorPort;
use crate::{wait, Attribute, Device, Driver, Ev3Error, Ev3Result};
use std::time::Duration;

/// EV3/NXT large servo motor
#[derive(Debug, Clone, Device)]
pub struct LargeMotor {
    driver: Driver,
}

impl LargeMotor {
    fn new(driver: Driver) -> Self {
        Self { driver }
    }

    findable!(
        "tacho-motor",
        ["lego-ev3-l-motor", "lego-nxt-motor"],
        MotorPort,
        "LargeMotor",
        "out"
    );
    tacho_motor!();
}

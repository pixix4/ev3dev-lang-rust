use crate::motors::{Motor, TachoMotor};
use crate::{Attribute, Device, Driver, Ev3Result, Findable};

/// EV3/NXT large servo motor
#[derive(Debug, Clone, Device, Findable, Motor, TachoMotor)]
#[class_name = "tacho-motor"]
#[driver_name = "lego-ev3-l-motor"]
#[port = "crate::motors::MotorPort"]
pub struct LargeMotor {
    driver: Driver,
}

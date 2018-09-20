#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod driver;
pub mod core;

pub mod servo_motor;
pub mod dc_motor;
pub mod tacho_motor;

pub mod color_sensor;
pub mod gyro_sensor;
pub mod touch_sensor;
pub mod infrared_sensor;
pub mod ultrasonic_sensor;

pub mod led;
pub mod power_supply;
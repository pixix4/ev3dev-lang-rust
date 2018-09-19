#![allow(dead_code)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod core;
pub mod driver;
pub mod servo_motor;
pub mod dc_motor;
pub mod tacho_motor;
pub mod color_sensor;
pub mod gyro_sensor;
pub mod led;
pub mod power_supply;
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort, TachoMotor};
use ev3dev_lang_rust::Ev3Result;

fn main() -> Ev3Result<()> {
    let large_motor = LargeMotor::get(MotorPort::OutA)?;
    let medium_motor = MediumMotor::get(MotorPort::OutB)?;

    // Set the initial speed so that the motors will move
    large_motor.set_speed_sp(300)?;
    medium_motor.set_speed_sp(300)?;

    large_motor.run_to_rel_pos(Some(360))?;
    medium_motor.run_to_rel_pos(Some(180))?;

    #[cfg(target_os = "linux")]
    large_motor.wait_until_not_moving(None);

    // If it does not matter which exact motor type is used, the wrapper `TachoMotor` can be used.

    let tacho_motor_1 = TachoMotor::get(MotorPort::OutA)?;
    let tacho_motor_2 = TachoMotor::get(MotorPort::OutB)?;

    tacho_motor_1.run_to_rel_pos(Some(360))?;
    tacho_motor_2.run_to_rel_pos(Some(180))?;

    #[cfg(target_os = "linux")]
    tacho_motor_1.wait_until_not_moving(None);

    Ok(())
}

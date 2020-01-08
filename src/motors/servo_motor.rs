//! The ServoMotor trait provides a uniform interface for using hobby type servo motors.

use super::Motor;
use crate::Ev3Result;

/// Remove power from the motor.
pub const COMMAND_RUN: &str = "run";

/// Drive servo to the position set in the position_sp attribute.
pub const COMMAND_FLOAT: &str = "float";

/// With normal polarity, a positive duty cycle will cause the motor to rotate clockwise.
pub const POLARITY_NORMAL: &str = "normal";

/// With inversed polarity, a positive duty cycle will cause the motor to rotate counter-clockwise.
pub const POLARITY_INVERSED: &str = "inversed";

/// Power is being sent to the motor.
pub const STATE_RUNNING: &str = "running";

/// The ServoMotor trait provides a uniform interface for using hobby type servo motors.
pub trait ServoMotor: Motor {
    /// Returns the current polarity of the motor.
    fn get_polarity(&self) -> Ev3Result<String> {
        self.get_attribute("polarity").get()
    }

    /// Sets the polarity of the motor.
    fn set_polarity(&self, polarity: &str) -> Ev3Result<()> {
        self.get_attribute("polarity").set_str_slice(polarity)
    }

    /// Returns the current max pulse setpoint.
    /// Used to set the pulse size in milliseconds for the signal
    /// that tells the servo to drive to the maximum (clockwise) position_sp.
    /// Default value is 2400. Valid values are 2300 to 2700.
    /// You must write to the position_sp attribute for changes to this attribute to take effect.
    fn get_max_pulse_sp(&self) -> Ev3Result<i32> {
        self.get_attribute("max_pulse_sp").get()
    }

    /// Sets the max pulse setpoint.
    /// Used to set the pulse size in milliseconds for the signal
    /// that tells the servo to drive to the maximum (clockwise) position_sp.
    /// Default value is 2400. Valid values are 2300 to 2700.
    /// You must write to the position_sp attribute for changes to this attribute to take effect.
    fn set_max_pulse_sp(&self, max_pulse_sp: i32) -> Ev3Result<()> {
        self.get_attribute("max_pulse_sp").set(max_pulse_sp)
    }

    /// Returns the current mid pulse setpoint.
    /// Used to set the pulse size in milliseconds for the signal
    /// that tells the servo to drive to the miniumum (counter-clockwise) position_sp.
    /// Default value is 600.
    /// Valid values are 300 to 700.
    ///  You must write to the position_sp attribute for changes to this attribute to take effect.
    fn get_mid_pulse_sp(&self) -> Ev3Result<i32> {
        self.get_attribute("mid_pulse_sp").get()
    }

    /// Sets the mid pulse setpoint.
    /// Used to set the pulse size in milliseconds for the signal
    /// that tells the servo to drive to the miniumum (counter-clockwise) position_sp.
    /// Default value is 600.
    /// Valid values are 300 to 700.
    ///  You must write to the position_sp attribute for changes to this attribute to take effect.
    fn set_mid_pulse_sp(&self, max_pulse_sp: i32) -> Ev3Result<()> {
        self.get_attribute("mid_pulse_sp").set(max_pulse_sp)
    }

    /// Returns the current min pulse setpoint.
    /// Used to set the pulse size in milliseconds for the signal
    /// that tells the servo to drive to the miniumum (counter-clockwise) position_sp.
    /// Default value is 600. Valid values are 300 to 700.
    /// You must write to the position_sp attribute for changes to this attribute to take effect.
    fn get_min_pulse_sp(&self) -> Ev3Result<i32> {
        self.get_attribute("min_pulse_sp").get()
    }
    /// Sets the min pulse setpoint.
    /// Used to set the pulse size in milliseconds for the signal
    /// that tells the servo to drive to the miniumum (counter-clockwise) position_sp.
    /// Default value is 600. Valid values are 300 to 700.
    /// You must write to the position_sp attribute for changes to this attribute to take effect.
    fn set_min_pulse_sp(&self, min_pulse_sp: i32) -> Ev3Result<()> {
        self.get_attribute("min_pulse_sp").set(min_pulse_sp)
    }

    /// Returns the current target position for the `run-to-abs-pos` and `run-to-rel-pos` commands. Units are in tacho counts.
    /// You can use the value returned by `counts_per_rot` to convert tacho counts to/from rotations or degrees.
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer).
    fn get_position_sp(&self) -> Ev3Result<i32> {
        self.get_attribute("position_sp").get()
    }

    /// Sets the target position for the `run-to-abs-pos` and `run-to-rel-pos` commands.
    /// Units are in tacho counts.
    /// You can use the value returned by `counts_per_rot` to convert tacho counts to/from rotations or degrees.
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer).
    fn set_position_sp(&self, position_sp: i32) -> Ev3Result<()> {
        self.get_attribute("position_sp").set(position_sp)
    }

    /// Returns the current the rate_sp at which the servo travels from 0 to 100.0%
    /// (half of the full range of the servo).
    /// Units are in milliseconds.
    ///
    /// ## Example:
    ///
    /// Setting the rate_sp to 1000 means that it will take a 180
    /// degree servo 2 second to move from 0 to 180 degrees.
    ///
    /// ## Note:
    ///
    /// Some servo controllers may not support this in which case
    /// reading and writing will fail with -EOPNOTSUPP.
    /// In continuous rotation servos, this value will affect the
    /// rate_sp at which the speed ramps up or down.
    fn get_rate_sp(&self) -> Ev3Result<i32> {
        self.get_attribute("rate_sp").get()
    }

    /// Sets the rate_sp at which the servo travels from 0 to 100.0%
    /// (half of the full range of the servo).
    /// Units are in milliseconds.
    ///
    /// ## Example:
    ///
    /// Setting the rate_sp to 1000 means that it will take a 180
    /// degree servo 2 second to move from 0 to 180 degrees.
    ///
    /// ## Note:
    ///
    /// Some servo controllers may not support this in which case
    /// reading and writing will fail with -EOPNOTSUPP.
    /// In continuous rotation servos, this value will affect the
    /// rate_sp at which the speed ramps up or down.
    fn set_rate_sp(&self, rate_sp: i32) -> Ev3Result<()> {
        self.get_attribute("rate_sp").set(rate_sp)
    }

    /// Returns a list of state flags.
    fn get_state(&self) -> Ev3Result<Vec<String>> {
        self.get_attribute("state").get_vec()
    }

    /// Power is being sent to the motor.
    fn is_running(&self) -> Ev3Result<bool> {
        Ok(self.get_state()?.iter().any(|state| state == STATE_RUNNING))
    }

    /// Drive servo to the position set in the `position_sp` attribute.
    fn run(&self) -> Ev3Result<()> {
        self.set_command(COMMAND_RUN)
    }

    /// Remove power from the motor.
    fn float(&self) -> Ev3Result<()> {
        self.set_command(COMMAND_FLOAT)
    }
}

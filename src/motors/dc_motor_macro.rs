//! The DcMotor provides a uniform interface for using
//! regular DC motors with no fancy controls or feedback.
//! This includes LEGO MINDSTORMS RCX motors and LEGO Power Functions motors.

/// The DcMotor provides a uniform interface for using
/// regular DC motors with no fancy controls or feedback.
/// This includes LEGO MINDSTORMS RCX motors and LEGO Power Functions motors.
#[macro_export]
macro_rules! dc_motor {
    () => {
        /// Causes the motor to run until another command is sent.
        pub const COMMAND_RUN_FOREVER: &'static str = "run-forever";

        /// Run the motor for the amount of time specified in `time_sp`
        /// and then stops the motor using the command specified by `stop_action`.
        pub const COMMAND_RUN_TIMED: &'static str = "run-timed";

        /// Runs the motor using the duty cycle specified by `duty_cycle_sp`.
        /// Unlike other run commands, changing `duty_cycle_sp` while running will take effect immediately.
        pub const COMMAND_RUN_DIRECT: &'static str = "run-direct";

        /// Stop any of the run commands before they are complete using the command specified by `stop_action`.
        pub const COMMAND_STOP: &'static str = "stop";

        /// A positive duty cycle will cause the motor to rotate clockwise.
        pub const POLARITY_NORMAL: &'static str = "normal";

        /// A positive duty cycle will cause the motor to rotate counter-clockwise.
        pub const POLARITY_INVERSED: &'static str = "inversed";

        /// Power is being sent to the motor.
        pub const STATE_RUNNING: &'static str = "running";

        /// The motor is ramping up or down and has not yet reached a pub constant output level.
        pub const STATE_RAMPING: &'static str = "ramping";

        /// Removes power from the motor. The motor will freely coast to a stop.
        pub const STOP_ACTION_COAST: &'static str = "coast";

        /// Removes power from the motor and creates a passive electrical load.
        /// This is usually done by shorting the motor terminals together.
        /// This load will absorb the energy from the rotation of the motors
        /// and cause the motor to stop more quickly than coasting.
        pub const STOP_ACTION_BRAKE: &'static str = "brake";

        /// Returns the current duty cycle of the motor. Units are percent. Values are -100 to 100.
        pub fn get_duty_cycle(&self) -> Ev3Result<i32> {
            self.get_attribute("duty_cycle").get()
        }

        /// Returns the current duty cycle setpoint of the motor. Units are in percent.
        /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
        pub fn get_duty_cycle_sp(&self) -> Ev3Result<i32> {
            self.get_attribute("duty_cycle_sp").get()
        }

        /// Sets the duty cycle setpoint of the motor. Units are in percent.
        /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
        pub fn set_duty_cycle_sp(&self, duty_cycle_sp: i32) -> Ev3Result<()> {
            self.get_attribute("duty_cycle_sp").set(duty_cycle_sp)
        }

        /// Returns the current polarity of the motor.
        pub fn get_polarity(&self) -> Ev3Result<String> {
            self.get_attribute("polarity").get()
        }

        /// Sets the polarity of the motor.
        pub fn set_polarity(&self, polarity: &str) -> Ev3Result<()> {
            self.get_attribute("polarity").set_str_slice(polarity)
        }

        /// Returns the current ramp up setpoint.
        /// Units are in milliseconds and must be positive. When set to a non-zero value,
        /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
        /// The actual ramp time is the ratio of the difference between the speed_sp
        /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
        pub fn get_ramp_up_sp(&self) -> Ev3Result<i32> {
            self.get_attribute("ramp_up_sp").get()
        }

        /// Sets the ramp up setpoint.
        /// Units are in milliseconds and must be positive. When set to a non-zero value,
        /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
        /// The actual ramp time is the ratio of the difference between the speed_sp
        /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
        pub fn set_ramp_up_sp(&self, ramp_up_sp: i32) -> Ev3Result<()> {
            self.get_attribute("ramp_up_sp").set(ramp_up_sp)
        }

        /// Returns the current ramp down setpoint.
        /// Units are in milliseconds and must be positive. When set to a non-zero value,
        /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
        /// The actual ramp time is the ratio of the difference between the speed_sp
        /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
        pub fn get_ramp_down_sp(&self) -> Ev3Result<i32> {
            self.get_attribute("ramp_down_sp").get()
        }

        /// Sets the ramp down setpoint.
        /// Units are in milliseconds and must be positive. When set to a non-zero value,
        /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
        /// The actual ramp time is the ratio of the difference between the speed_sp
        /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
        pub fn set_ramp_down_sp(&self, ramp_down_sp: i32) -> Ev3Result<()> {
            self.get_attribute("ramp_down_sp").set(ramp_down_sp)
        }

        /// Returns a list of state flags.
        pub fn get_state(&self) -> Ev3Result<Vec<String>> {
            self.get_attribute("state").get_vec()
        }

        /// Returns the current stop action.
        /// The value determines the motors behavior when command is set to stop.
        pub fn get_stop_action(&self) -> Ev3Result<String> {
            self.get_attribute("stop_action").get()
        }

        /// Sets the stop action.
        /// The value determines the motors behavior when command is set to stop.
        pub fn set_stop_action(&self, stop_action: &str) -> Ev3Result<()> {
            self.get_attribute("stop_action").set_str_slice(stop_action)
        }

        /// Returns the current amount of time the motor will run when using the run-timed command.
        /// Units are in milliseconds. Values must not be negative.
        pub fn get_time_sp(&self) -> Ev3Result<i32> {
            self.get_attribute("time_sp").get()
        }

        /// Sets the amount of time the motor will run when using the run-timed command.
        /// Units are in milliseconds. Values must not be negative.
        pub fn set_time_sp(&self, time_sp: i32) -> Ev3Result<()> {
            self.get_attribute("time_sp").set(time_sp)
        }

        /// Runs the motor using the duty cycle specified by `duty_cycle_sp`.
        /// Unlike other run commands, changing `duty_cycle_sp` while running will take effect immediately.
        pub fn run_direct(&self) -> Ev3Result<()> {
            self.set_command(Self::COMMAND_RUN_DIRECT)
        }

        /// Causes the motor to run until another command is sent.
        pub fn run_forever(&self) -> Ev3Result<()> {
            self.set_command(Self::COMMAND_RUN_FOREVER)
        }

        /// Run the motor for the amount of time specified in `time_sp`
        /// and then stops the motor using the command specified by `stop_action`.
        pub fn run_timed(&self, time_sp: Option<Duration>) -> Ev3Result<()> {
            if let Some(duration) = time_sp {
                let p = duration.as_millis() as i32;
                self.set_time_sp(p)?;
            }
            self.set_command(Self::COMMAND_RUN_TIMED)
        }

        /// Stop any of the run commands before they are complete using the command specified by `stop_action`.
        pub fn stop(&self) -> Ev3Result<()> {
            self.set_command(Self::COMMAND_STOP)
        }

        /// Power is being sent to the motor.
        pub fn is_running(&self) -> Ev3Result<bool> {
            Ok(self
                .get_state()?
                .iter()
                .any(|state| state == Self::STATE_RUNNING))
        }

        /// The motor is ramping up or down and has not yet reached a pub constant output level.
        pub fn is_ramping(&self) -> Ev3Result<bool> {
            Ok(self
                .get_state()?
                .iter()
                .any(|state| state == Self::STATE_RAMPING))
        }
    };
}

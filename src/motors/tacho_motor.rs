use crate::motors::Motor;
use crate::{wait, Ev3Result};

use std::time::Duration;

/// Causes the motor to run until another command is sent.
pub const RUN_FOREVER: &str = "run-forever";

/// Runs the motor to an absolute position specified by `position_sp`
/// and then stops the motor using the command specified in `stop_action`.
pub const RUN_TO_ABS_POS: &str = "run-to-abs-pos";

/// Runs the motor to a position relative to the current position value.
/// The new position will be current `position` + `position_sp`.
/// When the new position is reached, the motor will stop using the command specified by `stop_action`.
pub const RUN_TO_REL_POS: &str = "run-to-rel-pos";

/// Run the motor for the amount of time specified in `time_sp`
/// and then stops the motor using the command specified by `stop_action`.
pub const RUN_TIMED: &str = "run-timed";

/// Runs the motor using the duty cycle specified by `duty_cycle_sp`.
/// Unlike other run commands, changing `duty_cycle_sp` while running will take effect immediately.
pub const RUN_DIRECT: &str = "run-direct";

/// Stop any of the run commands before they are complete using the command specified by `stop_action`.
pub const STOP: &str = "stop";

/// Resets all of the motor parameter attributes to their default values.
/// This will also have the effect of stopping the motor.
pub const RESET: &str = "reset";

/// A positive duty cycle will cause the motor to rotate clockwise.
pub const POLARITY_NORMAL: &str = "normal";

/// A positive duty cycle will cause the motor to rotate counter-clockwise.
pub const POLARITY: &str = "reversed";

/// Power is being sent to the motor.
pub const STATE_RUNNING: &str = "running";

/// The motor is ramping up or down and has not yet reached a pub constant output level.
pub const STATE_RAMPING: &str = "ramping";

/// The motor is not turning, but rather attempting to hold a fixed position.
pub const STATE_HOLDING: &str = "holding";

/// The motor is turning as fast as possible, but cannot reach its `speed_sp`.
pub const STATE_OVERLOADED: &str = "overloaded";

/// The motor is trying to run but is not turning at all.
pub const STATE_STALLED: &str = "stalled";

/// Removes power from the motor. The motor will freely coast to a stop.
pub const STOP_ACTION_COAST: &str = "coast";

/// Removes power from the motor and creates a passive electrical load.
/// This is usually done by shorting the motor terminals together.
/// This load will absorb the energy from the rotation of the motors
/// and cause the motor to stop more quickly than coasting.
pub const STOP_ACTION_BRAKE: &str = "brake";

/// Causes the motor to actively try to hold the current position.
/// If an external force tries to turn the motor, the motor will “push back” to maintain its position.
pub const STOP_ACTION_HOLD: &str = "hold";

/// The `tacho-motor` class provides a uniform interface for using motors with positional
/// and directional feedback such as the EV3 and NXT motors.
/// This feedback allows for precise control of the motors.
pub trait TachoMotor: Motor {
    /// Returns the number of tacho counts in one rotation of the motor.
    /// Tacho counts are used by the position and speed attributes,
    /// so you can use this value to convert from rotations or degrees to tacho counts.
    /// (rotation motors only)
    fn get_count_per_rot(&self) -> Ev3Result<i32> {
        self.get_attribute("count_per_rot").get()
    }

    /// Returns the number of tacho counts in one meter of travel of the motor.
    /// Tacho counts are used by the position and speed attributes,
    /// so you can use this value to convert from distance to tacho counts.
    /// (linear motors only)
    fn get_count_per_m(&self) -> Ev3Result<i32> {
        self.get_attribute("count_per_m").get()
    }

    /// Returns the number of tacho counts in the full travel of the motor.
    /// When combined with the count_per_m atribute,
    /// you can use this value to calculate the maximum travel distance of the motor.
    /// (linear motors only)
    fn get_full_travel_count(&self) -> Ev3Result<i32> {
        self.get_attribute("full_travel_count").get()
    }

    /// Returns the current duty cycle of the motor. Units are percent. Values are -100 to 100.
    fn get_duty_cycle(&self) -> Ev3Result<i32> {
        self.get_attribute("duty_cycle").get()
    }

    /// Returns the current duty cycle setpoint of the motor. Units are in percent.
    /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
    fn get_duty_cycle_sp(&self) -> Ev3Result<i32> {
        self.get_attribute("duty_cycle_sp").get()
    }

    /// Sets the duty cycle setpoint of the motor. Units are in percent.
    /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
    fn set_duty_cycle_sp(&self, duty_cycle: i32) -> Ev3Result<()> {
        self.get_attribute("duty_cycle_sp").set(duty_cycle)
    }

    /// Returns the current polarity of the motor.
    fn get_polarity(&self) -> Ev3Result<String> {
        self.get_attribute("polarity").get()
    }

    /// Sets the polarity of the motor.
    fn set_polarity(&self, polarity: &str) -> Ev3Result<()> {
        self.get_attribute("polarity").set_str_slice(polarity)
    }

    /// Returns the current position of the motor in pulses of the rotary encoder.
    /// When the motor rotates clockwise, the position will increase.
    /// Likewise, rotating counter-clockwise causes the position to decrease.
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer)
    fn get_position(&self) -> Ev3Result<i32> {
        self.get_attribute("position").get()
    }

    /// Sets the current position of the motor in pulses of the rotary encoder.
    /// When the motor rotates clockwise, the position will increase.
    /// Likewise, rotating counter-clockwise causes the position to decrease.
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer)
    fn set_position(&self, position: i32) -> Ev3Result<()> {
        self.get_attribute("position").set(position)
    }

    /// Returns the proportional pub constant for the position PID.
    fn get_hold_pid_kp(&self) -> Ev3Result<f32> {
        self.get_attribute("hold_pid_kp").get()
    }

    /// Sets the proportional pub constant for the position PID.
    fn set_hold_pid_kp(&self, kp: f32) -> Ev3Result<()> {
        self.get_attribute("hold_pid_kp").set(kp)
    }

    /// Returns the integral pub constant for the position PID.
    fn get_hold_pid_ki(&self) -> Ev3Result<f32> {
        self.get_attribute("hold_pid_ki").get()
    }

    /// Sets the integral pub constant for the position PID.
    fn set_hold_pid_ki(&self, ki: f32) -> Ev3Result<()> {
        self.get_attribute("hold_pid_ki").set(ki)
    }

    /// Returns the derivative pub constant for the position PID.
    fn get_hold_pid_kd(&self) -> Ev3Result<f32> {
        self.get_attribute("hold_pid_kd").get()
    }

    /// Sets the derivative pub constant for the position PID.
    fn set_hold_pid_kd(&self, kd: f32) -> Ev3Result<()> {
        self.get_attribute("hold_pid_kd").set(kd)
    }

    /// Returns the maximum value that is accepted by the `speed_sp`
    /// attribute. This value is the speed of the motor at 9V with no load.
    /// Note: The actual maximum obtainable speed will be less than this
    /// and will depend on battery voltage and mechanical load on the motor.
    fn get_max_speed(&self) -> Ev3Result<i32> {
        self.get_attribute("max_speed").get()
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

    /// Returns the current motor speed in tacho counts per second.
    /// Note, this is not necessarily degrees (although it is for LEGO motors).
    /// Use the `count_per_rot` attribute to convert this value to RPM or deg/sec.
    fn get_speed(&self) -> Ev3Result<i32> {
        self.get_attribute("speed").get()
    }

    /// Returns the target speed in tacho counts per second used for all run-* commands except run-direct.
    /// A negative value causes the motor to rotate in reverse
    /// with the exception of run-to-*-pos commands where the sign is ignored.
    /// Use the `count_per_rot` attribute to convert RPM or deg/sec to tacho counts per second.
    /// Use the `count_per_m` attribute to convert m/s to tacho counts per second.
    fn get_speed_sp(&self) -> Ev3Result<i32> {
        self.get_attribute("speed_sp").get()
    }

    /// Sets the target speed in tacho counts per second used for all run-* commands except run-direct.
    /// A negative value causes the motor to rotate in reverse
    /// with the exception of run-to-*-pos commands where the sign is ignored.
    /// Use the `count_per_rot` attribute to convert RPM or deg/sec to tacho counts per second.
    /// Use the `count_per_m` attribute to convert m/s to tacho counts per second.
    fn set_speed_sp(&self, speed_sp: i32) -> Ev3Result<()> {
        self.get_attribute("speed_sp").set(speed_sp)
    }

    /// Returns the current ramp up setpoint.
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
    fn get_ramp_up_sp(&self) -> Ev3Result<i32> {
        self.get_attribute("ramp_up_sp").get()
    }

    /// Sets the ramp up setpoint.
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
    fn set_ramp_up_sp(&self, ramp_up_sp: i32) -> Ev3Result<()> {
        self.get_attribute("ramp_up_sp").set(ramp_up_sp)
    }

    /// Returns the current ramp down setpoint.
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
    fn get_ramp_down_sp(&self) -> Ev3Result<i32> {
        self.get_attribute("ramp_down_sp").get()
    }

    /// Sets the ramp down setpoint.
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
    fn set_ramp_down_sp(&self, ramp_down_sp: i32) -> Ev3Result<()> {
        self.get_attribute("ramp_down_sp").set(ramp_down_sp)
    }

    /// Returns the proportional pub constant for the speed regulation PID.
    fn get_speed_pid_kp(&self) -> Ev3Result<f32> {
        self.get_attribute("speed_pid_kp").get()
    }

    /// Sets the proportional pub constant for the speed regulation PID.
    fn set_speed_pid_kp(&self, kp: f32) -> Ev3Result<()> {
        self.get_attribute("speed_pid_kp").set(kp)
    }

    /// Returns the integral pub constant for the speed regulation PID.
    fn get_speed_pid_ki(&self) -> Ev3Result<f32> {
        self.get_attribute("speed_pid_ki").get()
    }

    /// Sets the integral pub constant for the speed regulation PID.
    fn set_speed_pid_ki(&self, ki: f32) -> Ev3Result<()> {
        self.get_attribute("speed_pid_ki").set(ki)
    }

    /// Returns the derivative pub constant for the speed regulation PID.
    fn get_speed_pid_kd(&self) -> Ev3Result<f32> {
        self.get_attribute("speed_pid_kd").get()
    }

    /// Sets the derivative pub constant for the speed regulation PID.
    fn set_speed_pid_kd(&self, kd: f32) -> Ev3Result<()> {
        self.get_attribute("speed_pid_kd").set(kd)
    }

    /// Returns a list of state flags.
    fn get_state(&self) -> Ev3Result<Vec<String>> {
        self.get_attribute("state").get_vec()
    }

    /// Returns the current stop action.
    /// The value determines the motors behavior when command is set to stop.
    fn get_stop_action(&self) -> Ev3Result<String> {
        self.get_attribute("stop_action").get()
    }

    /// Sets the stop action.
    /// The value determines the motors behavior when command is set to stop.
    fn set_stop_action(&self, stop_action: &str) -> Ev3Result<()> {
        self.get_attribute("stop_action").set_str_slice(stop_action)
    }

    /// Returns a list of stop actions supported by the motor controller.
    fn get_stop_actions(&self) -> Ev3Result<Vec<String>> {
        self.get_attribute("stop_actions").get_vec()
    }

    /// Returns the current amount of time the motor will run when using the run-timed command.
    /// Units are in milliseconds. Values must not be negative.
    fn get_time_sp(&self) -> Ev3Result<i32> {
        self.get_attribute("time_sp").get()
    }

    /// Sets the amount of time the motor will run when using the run-timed command.
    /// Units are in milliseconds. Values must not be negative.
    fn set_time_sp(&self, time_sp: i32) -> Ev3Result<()> {
        self.get_attribute("time_sp").set(time_sp)
    }

    /// Runs the motor using the duty cycle specified by `duty_cycle_sp`.
    /// Unlike other run commands, changing `duty_cycle_sp` while running will take effect immediately.
    fn run_direct(&self) -> Ev3Result<()> {
        self.set_command(RUN_DIRECT)
    }

    /// Causes the motor to run until another command is sent.
    fn run_forever(&self) -> Ev3Result<()> {
        self.set_command(RUN_FOREVER)
    }

    /// Runs the motor to an absolute position specified by `position_sp`
    /// and then stops the motor using the command specified in `stop_action`.
    fn run_to_abs_pos(&self, position_sp: Option<i32>) -> Ev3Result<()> {
        if let Some(p) = position_sp {
            self.set_position_sp(p)?;
        }
        self.set_command(RUN_TO_ABS_POS)
    }

    /// Runs the motor to a position relative to the current position value.
    /// The new position will be current `position` + `position_sp`.
    /// When the new position is reached, the motor will stop using the command specified by `stop_action`.
    fn run_to_rel_pos(&self, position_sp: Option<i32>) -> Ev3Result<()> {
        if let Some(p) = position_sp {
            self.set_position_sp(p)?;
        }
        self.set_command(RUN_TO_REL_POS)
    }

    /// Run the motor for the amount of time specified in `time_sp`
    /// and then stops the motor using the command specified by `stop_action`.
    fn run_timed(&self, time_sp: Option<Duration>) -> Ev3Result<()> {
        if let Some(duration) = time_sp {
            let p = duration.as_millis() as i32;
            self.set_time_sp(p)?;
        }
        self.set_command(RUN_TIMED)
    }

    /// Stop any of the run commands before they are complete using the command specified by `stop_action`.
    fn stop(&self) -> Ev3Result<()> {
        self.set_command(STOP)
    }

    /// Resets all of the motor parameter attributes to their default values.
    /// This will also have the effect of stopping the motor.
    fn reset(&self) -> Ev3Result<()> {
        self.set_command(RESET)
    }

    /// Power is being sent to the motor.
    fn is_running(&self) -> Ev3Result<bool> {
        Ok(self.get_state()?.iter().any(|state| state == STATE_RUNNING))
    }

    /// The motor is ramping up or down and has not yet reached a pub constant output level.
    fn is_ramping(&self) -> Ev3Result<bool> {
        Ok(self.get_state()?.iter().any(|state| state == STATE_RAMPING))
    }

    /// The motor is not turning, but rather attempting to hold a fixed position.
    fn is_holding(&self) -> Ev3Result<bool> {
        Ok(self.get_state()?.iter().any(|state| state == STATE_HOLDING))
    }

    /// The motor is turning as fast as possible, but cannot reach its `speed_sp`.
    fn is_overloaded(&self) -> Ev3Result<bool> {
        Ok(self
            .get_state()?
            .iter()
            .any(|state| state == STATE_OVERLOADED))
    }

    /// The motor is trying to run but is not turning at all.
    fn is_stalled(&self) -> Ev3Result<bool> {
        Ok(self.get_state()?.iter().any(|state| state == STATE_STALLED))
    }

    /// Wait until condition `cond` returns true or the `timeout` is reached.
    /// The condition is checked when to the `state` attribute has changed.
    /// If the `timeout` is `None` it will wait an infinite time.
    fn wait<F>(&self, cond: F, timeout: Option<Duration>) -> bool
    where
        F: Fn() -> bool,
    {
        let fd = self.get_attribute("state").get_raw_fd();
        wait::wait(fd, cond, timeout)
    }

    /// Wait while the `state` is in the vector `self.get_state()` or the `timeout` is reached.
    /// If the `timeout` is `None` it will wait an infinite time.
    fn wait_while(&self, state: &str, timeout: Option<Duration>) -> bool {
        let cond = || {
            self.get_state()
                .unwrap_or_else(|_| vec![])
                .iter()
                .all(|s| s != state)
        };
        self.wait(cond, timeout)
    }

    /// Wait until the `state` is in the vector `self.get_state()` or the `timeout` is reached.
    /// If the `timeout` is `None` it will wait an infinite time.
    fn wait_until(&self, state: &str, timeout: Option<Duration>) -> bool {
        let cond = || {
            self.get_state()
                .unwrap_or_else(|_| vec![])
                .iter()
                .any(|s| s == state)
        };
        self.wait(cond, timeout)
    }

    /// Wait until the motor is not moving or the timeout is reached.
    /// This is euqal to `wait_until(STATE_RUNNING, timeout)`
    /// If the `timeout` is `None` it will wait an infinite time.
    fn wait_until_not_moving(&self, timeout: Option<Duration>) -> bool {
        self.wait_while(STATE_RUNNING, timeout)
    }
}

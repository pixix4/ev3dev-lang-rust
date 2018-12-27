use core::Motor;
use core::Device;
use core::MotorPort;
use driver::Driver;
use driver::Attribute;
use driver::AttributeResult;

/// Causes the motor to run until another command is sent.
pub const RUN_FOREVER: &'static str = "run-forever";

/// Runs the motor to an absolute position specified by `position_sp`
/// and then stops the motor using the command specified in `stop_action`.
pub const RUN_TO_ABS_POS: &'static str = "run-to-abs-pos";

/// Runs the motor to a position relative to the current position value.
/// The new position will be current `position` + `position_sp`.
/// When the new position is reached, the motor will stop using the command specified by `stop_action`.
pub const RUN_TO_REL_POS: &'static str = "run-to-rel-pos";

/// Run the motor for the amount of time specified in `time_sp`
/// and then stops the motor using the command specified by `stop_action`.
pub const RUN_TIMED: &'static str = "run-timed";

/// Runs the motor using the duty cycle specified by `duty_cycle_sp`.
/// Unlike other run commands, changing `duty_cycle_sp` while running will take effect immediately.
pub const RUN_DIRECT: &'static str = "run-direct";

/// Stop any of the run commands before they are complete using the command specified by `stop_action`.
pub const STOP: &'static str = "stop";

/// Resets all of the motor parameter attributes to their default values.
/// This will also have the effect of stopping the motor.
pub const RESET: &'static str = "reset";

/// A positive duty cycle will cause the motor to rotate clockwise.
pub const POLARITY_NORMAL: &'static str = "normal";

/// A positive duty cycle will cause the motor to rotate counter-clockwise.
pub const POLARITY: &'static str = "reversed";

/// Power is being sent to the motor.
pub const STATE_RUNNING: &'static str = "running";

/// The motor is ramping up or down and has not yet reached a constant output level.
pub const STATE_RAMPING: &'static str = "ramping";

/// The motor is not turning, but rather attempting to hold a fixed position.
pub const STATE_HOLDING: &'static str = "holding";

/// The motor is turning as fast as possible, but cannot reach its `speed_sp`.
pub const STATE_OVERLOADED: &'static str = "overloaded";

/// The motor is trying to run but is not turning at all.
pub const STATE_STALLED: &'static str = "stalled";

/// Removes power from the motor. The motor will freely coast to a stop.
pub const STOP_ACTION_COAST: &'static str = "coast";

/// Removes power from the motor and creates a passive electrical load.
/// This is usually done by shorting the motor terminals together.
/// This load will absorb the energy from the rotation of the motors
/// and cause the motor to stop more quickly than coasting.
pub const STOP_ACTION_BRAKE: &'static str = "brake";

/// Causes the motor to actively try to hold the current position.
/// If an external force tries to turn the motor, the motor will “push back” to maintain its position.
pub const STOP_ACTION_HOLD: &'static str = "hold";

/// The `tacho-motor` class provides a uniform interface for using motors with positional
/// and directional feedback such as the EV3 and NXT motors.
/// This feedback allows for precise control of the motors.
pub trait TachoMotor: Motor {
    /// Returns the number of tacho counts in one rotation of the motor.
    /// Tacho counts are used by the position and speed attributes,
    /// so you can use this value to convert from rotations or degrees to tacho counts.
    /// (rotation motors only)
    fn get_count_per_rot(&mut self) -> AttributeResult<isize> {
        self.get_attribute("count_per_rot").get_int()
    }

    /// Returns the number of tacho counts in one meter of travel of the motor.
    /// Tacho counts are used by the position and speed attributes,
    /// so you can use this value to convert from distance to tacho counts.
    /// (linear motors only)
    fn get_count_per_m(&mut self) -> AttributeResult<isize> {
        self.get_attribute("count_per_m").get_int()
    }

    /// Returns the number of tacho counts in the full travel of the motor.
    /// When combined with the count_per_m atribute,
    /// you can use this value to calculate the maximum travel distance of the motor.
    /// (linear motors only)
    fn get_full_travel_count(&mut self) -> AttributeResult<isize> {
        self.get_attribute("full_travel_count").get_int()
    }

    /// Returns the current duty cycle of the motor. Units are percent. Values are -100 to 100.
    fn get_duty_cycle(&mut self) -> AttributeResult<isize> {
        self.get_attribute("duty_cycle").get_int()
    }

    /// Returns the current duty cycle setpoint of the motor. Units are in percent.
    /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
    fn get_duty_cycle_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("duty_cycle_sp").get_int()
    }

    /// Sets the duty cycle setpoint of the motor. Units are in percent.
    /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
    fn set_duty_cycle_sp(&mut self, duty_cycle: isize) -> AttributeResult<()> {
        self.get_attribute("duty_cycle_sp").set_int(duty_cycle)
    }

    /// Returns the current polarity of the motor.
    fn get_polarity(&mut self) -> AttributeResult<String> {
        self.get_attribute("polarity").get_str()
    }

    /// Sets the polarity of the motor.
    fn set_polarity(&mut self, polarity: String) -> AttributeResult<()> {
        self.get_attribute("polarity").set_str(polarity)
    }

    /// Returns the current position of the motor in pulses of the rotary encoder.
    /// When the motor rotates clockwise, the position will increase.
    /// Likewise, rotating counter-clockwise causes the position to decrease.
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer)
    fn get_position(&mut self) -> AttributeResult<isize> {
        self.get_attribute("position").get_int()
    }

    /// Sets the current position of the motor in pulses of the rotary encoder.
    /// When the motor rotates clockwise, the position will increase.
    /// Likewise, rotating counter-clockwise causes the position to decrease.
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer)
    fn set_position(&mut self, position: isize) -> AttributeResult<()> {
        self.get_attribute("position").set_int(position)
    }

    /// Returns the proportional constant for the position PID.
    fn get_hold_pid_kp(&mut self) -> AttributeResult<f32> {
        self.get_attribute("hold_pid_kp").get_float()
    }

    /// Sets the proportional constant for the position PID.
    fn set_hold_pid_kp(&mut self, kp: f32) -> AttributeResult<()> {
        self.get_attribute("hold_pid_kp").set_float(kp)
    }

    /// Returns the integral constant for the position PID.
    fn get_hold_pid_ki(&mut self) -> AttributeResult<f32> {
        self.get_attribute("hold_pid_ki").get_float()
    }

    /// Sets the integral constant for the position PID.
    fn set_hold_pid_ki(&mut self, ki: f32) -> AttributeResult<()> {
        self.get_attribute("hold_pid_ki").set_float(ki)
    }

    /// Returns the derivative constant for the position PID.
    fn get_hold_pid_kd(&mut self) -> AttributeResult<f32> {
        self.get_attribute("hold_pid_kd").get_float()
    }

    /// Sets the derivative constant for the position PID.
    fn set_hold_pid_kd(&mut self, kd: f32) -> AttributeResult<()> {
        self.get_attribute("hold_pid_kd").set_float(kd)
    }

    /// Returns the maximum value that is accepted by the `speed_sp`
    /// attribute. This value is the speed of the motor at 9V with no load.
    /// Note: The actual maximum obtainable speed will be less than this
    /// and will depend on battery voltage and mechanical load on the motor.
    fn get_max_speed(&mut self) -> AttributeResult<isize> {
        self.get_attribute("max_speed").get_int()
    }

    /// Returns the current target position for the `run-to-abs-pos` and `run-to-rel-pos` commands. Units are in tacho counts.
    /// You can use the value returned by `counts_per_rot` to convert tacho counts to/from rotations or degrees.
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer).
    fn get_position_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("position_sp").get_int()
    }

    /// Sets the target position for the `run-to-abs-pos` and `run-to-rel-pos` commands.
    /// Units are in tacho counts.
    /// You can use the value returned by `counts_per_rot` to convert tacho counts to/from rotations or degrees.
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer).
    fn set_position_sp(&mut self, position_sp: isize) -> AttributeResult<()> {
        self.get_attribute("position_sp").set_int(position_sp)
    }

    /// Returns the current motor speed in tacho counts per second.
    /// Note, this is not necessarily degrees (although it is for LEGO motors).
    /// Use the `count_per_rot` attribute to convert this value to RPM or deg/sec.
    fn get_speed(&mut self) -> AttributeResult<isize> {
        self.get_attribute("speed").get_int()
    }

    /// Returns the target speed in tacho counts per second used for all run-* commands except run-direct.
    /// A negative value causes the motor to rotate in reverse
    /// with the exception of run-to-*-pos commands where the sign is ignored.
    /// Use the `count_per_rot` attribute to convert RPM or deg/sec to tacho counts per second.
    /// Use the `count_per_m` attribute to convert m/s to tacho counts per second.
    fn get_speed_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("speed_sp").get_int()
    }

    /// Sets the target speed in tacho counts per second used for all run-* commands except run-direct.
    /// A negative value causes the motor to rotate in reverse
    /// with the exception of run-to-*-pos commands where the sign is ignored.
    /// Use the `count_per_rot` attribute to convert RPM or deg/sec to tacho counts per second.
    /// Use the `count_per_m` attribute to convert m/s to tacho counts per second.
    fn set_speed_sp(&mut self, speed_sp: isize) -> AttributeResult<()> {
        self.get_attribute("speed_sp").set_int(speed_sp)
    }

    /// Returns the current ramp up setpoint.
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
    fn get_ramp_up_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("ramp_up_sp").get_int()
    }

    /// Sets the ramp up setpoint.
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
    fn set_ramp_up_sp(&mut self, ramp_up_sp: isize) -> AttributeResult<()> {
        self.get_attribute("ramp_up_sp").set_int(ramp_up_sp)
    }

    /// Returns the current ramp down setpoint.
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
    fn get_ramp_down_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("ramp_down_sp").get_int()
    }

    /// Sets the ramp down setpoint.
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
    fn set_ramp_down_sp(&mut self, ramp_down_sp: isize) -> AttributeResult<()> {
        self.get_attribute("ramp_down_sp").set_int(ramp_down_sp)
    }

    /// Returns the proportional constant for the speed regulation PID.
    fn get_speed_pid_kp(&mut self) -> AttributeResult<f32> {
        self.get_attribute("speed_pid_kp").get_float()
    }

    /// Sets the proportional constant for the speed regulation PID.
    fn set_speed_pid_kp(&mut self, kp: f32) -> AttributeResult<()> {
        self.get_attribute("speed_pid_kp").set_float(kp)
    }

    /// Returns the integral constant for the speed regulation PID.
    fn get_speed_pid_ki(&mut self) -> AttributeResult<f32> {
        self.get_attribute("speed_pid_ki").get_float()
    }

    /// Sets the integral constant for the speed regulation PID.
    fn set_speed_pid_ki(&mut self, ki: f32) -> AttributeResult<()> {
        self.get_attribute("speed_pid_ki").set_float(ki)
    }

    /// Returns the derivative constant for the speed regulation PID.
    fn get_speed_pid_kd(&mut self) -> AttributeResult<f32> {
        self.get_attribute("speed_pid_kd").get_float()
    }

    /// Sets the derivative constant for the speed regulation PID.
    fn set_speed_pid_kd(&mut self, kd: f32) -> AttributeResult<()> {
        self.get_attribute("speed_pid_kd").set_float(kd)
    }

    /// Returns a list of state flags.
    fn get_state(&mut self) -> AttributeResult<Vec<String>> {
        self.get_attribute("state").get_vec()
    }

    /// Returns the current stop action.
    /// The value determines the motors behavior when command is set to stop.
    fn get_stop_action(&mut self) -> AttributeResult<String> {
        self.get_attribute("stop_action").get_str()
    }

    /// Sets the stop action.
    /// The value determines the motors behavior when command is set to stop.
    fn set_stop_action(&mut self, stop_action: String) -> AttributeResult<()> {
        self.get_attribute("stop_action").set_str(stop_action)
    }

    /// Returns a list of stop actions supported by the motor controller.
    fn get_stop_actions(&mut self) -> AttributeResult<Vec<String>> {
        self.get_attribute("stop_actions").get_vec()
    }

    /// Returns the current amount of time the motor will run when using the run-timed command.
    /// Units are in milliseconds. Values must not be negative.
    fn get_time_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("time_sp").get_int()
    }


    /// Sets the amount of time the motor will run when using the run-timed command.
    /// Units are in milliseconds. Values must not be negative.
    fn set_time_sp(&mut self, time_sp: isize) -> AttributeResult<()> {
        self.get_attribute("time_sp").set_int(time_sp)
    }

    fn run_direct(&mut self) -> AttributeResult<()> {
        self.set_command(String::from(RUN_DIRECT))
    }
    fn run_forever(&mut self) -> AttributeResult<()> {
        self.set_command(String::from(RUN_FOREVER))
    }

    fn run_to_abs_pos(&mut self, position_sp: Option<isize>) -> AttributeResult<()> {
        if let Some(p) = position_sp {
            self.set_position_sp(p)?;
        }
        self.set_command(String::from(RUN_TO_ABS_POS))
    }

    fn run_to_rel_pos(&mut self, position_sp: Option<isize>) -> AttributeResult<()> {
        if let Some(p) = position_sp {
            self.set_position_sp(p)?;
        }
        self.set_command(String::from(RUN_TO_REL_POS))
    }

    fn run_timed(&mut self, time_sp: Option<isize>) -> AttributeResult<()> {
        if let Some(p) = time_sp {
            self.set_time_sp(p)?;
        }
        self.set_command(String::from(RUN_TIMED))
    }
    fn stop(&mut self) -> AttributeResult<()> {
        self.set_command(String::from(STOP))
    }
    fn reset(&mut self) -> AttributeResult<()> {
        self.set_command(String::from(RESET))
    }


    fn is_running(&mut self) -> AttributeResult<bool> {
        let states = self.get_state()?;
        for state in states {
            if state == STATE_RUNNING {
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn is_ramping(&mut self) -> AttributeResult<bool> {
        let states = self.get_state()?;
        for state in states {
            if state == STATE_RAMPING {
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn is_holding(&mut self) -> AttributeResult<bool> {
        let states = self.get_state()?;
        for state in states {
            if state == STATE_HOLDING {
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn is_overloaded(&mut self) -> AttributeResult<bool> {
        let states = self.get_state()?;
        for state in states {
            if state == STATE_OVERLOADED {
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn is_stalled(&mut self) -> AttributeResult<bool> {
        let states = self.get_state()?;
        for state in states {
            if state == STATE_STALLED {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

pub struct LargeMotor {
    driver: Driver
}

impl Motor for LargeMotor {}

impl TachoMotor for LargeMotor {}

impl Device for LargeMotor {
    fn get_attribute(&mut self, name: &str) -> &Attribute {
        self.driver.get_attribute(name)
    }
}

impl LargeMotor {
    /// Try to get a `LargeMotor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: MotorPort) -> Option<LargeMotor> {
        if let Some(name) = Driver::find_name_by_port_and_driver("tacho-motor", &port, "lego-ev3-l-motor") {
            return Some(LargeMotor {
                driver: Driver::new(String::from("tacho-motor"), name)
            });
        }

        None
    }

    /// Try to find a `LargeMotor`. Only returns a motor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Option<LargeMotor> {
        if let Some(name) = Driver::find_name_by_driver("tacho-motor", "lego-ev3-l-motor") {
            return Some(LargeMotor {
                driver: Driver::new(String::from("tacho-motor"), name)
            });
        }

        None
    }

    /// Extract list of connected 'LargeMotor'
    pub fn list() -> Vec<LargeMotor> {
        Driver::find_names_by_driver("tacho_motor", "lego-ev3-l-motor")
            .iter()
            .map(|name| LargeMotor{ driver: Driver::new(String::from("tacho-motor"), name.to_string())})
            .collect()
    }
}

pub struct MediumMotor {
    driver: Driver
}

impl Motor for MediumMotor {}

impl TachoMotor for MediumMotor {}

impl Device for MediumMotor {
    fn get_attribute(&mut self, name: &str) -> &Attribute {
        self.driver.get_attribute(name)
    }
}

impl MediumMotor {
    /// Try to get a `MediumMotor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: MotorPort) -> Option<MediumMotor> {
        if let Some(name) = Driver::find_name_by_port_and_driver("tacho-motor", &port, "lego-ev3-m-motor") {
            return Some(MediumMotor {
                driver: Driver::new(String::from("tacho-motor"), name)
            });
        }

        None
    }

    /// Try to find a `MediumMotor`. Only returns a motor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Option<MediumMotor> {
        if let Some(name) = Driver::find_name_by_driver("tacho-motor", "lego-ev3-m-motor") {
            return Some(MediumMotor {
                driver: Driver::new(String::from("tacho-motor"), name)
            });
        }

        None
    }

    /// Extract list of connected 'MediumMotor'
    pub fn list() -> Vec<MediumMotor> {
        Driver::find_names_by_driver("tacho_motor", "lego-ev3-m-motor")
            .iter()
            .map(|name| MediumMotor{ driver: Driver::new(String::from("tacho-motor"), name.to_string())})
            .collect()
    }
}
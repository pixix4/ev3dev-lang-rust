//! The TachoMotor provides a uniform interface for using motors with positional
//! and directional feedback such as the EV3 and NXT motors.
//! This feedback allows for precise control of the motors.

use std::time::Duration;

use crate::{Ev3Error, Ev3Result};

use super::{LargeMotor, MediumMotor, MotorPort};

#[derive(Debug, Clone)]
enum TachoMotorInner {
    LargeMotor { motor: LargeMotor },
    MediumMotor { motor: MediumMotor },
}

/// The TachoMotor provides a uniform interface for using motors with positional
/// and directional feedback such as the EV3 and NXT motors.
/// This feedback allows for precise control of the motors.
/// EV3/NXT large servo motor
#[derive(Debug, Clone)]
pub struct TachoMotor {
    inner: TachoMotorInner,
}

impl TachoMotor {
    /// Try to get a `Self` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn get(port: MotorPort) -> Ev3Result<Self> {
        let large_motor = LargeMotor::get(port);
        if let Ok(motor) = large_motor {
            return Ok(TachoMotor {
                inner: TachoMotorInner::LargeMotor { motor },
            });
        }

        let medium_motor = MediumMotor::get(port);
        if let Ok(motor) = medium_motor {
            return Ok(TachoMotor {
                inner: TachoMotorInner::MediumMotor { motor },
            });
        }

        Err(Ev3Error::InternalError {
            msg: "Could not find a tacho motor at requested port!".to_owned(),
        })
    }

    /// Try to find a `Self`. Only returns a motor if their is exactly one connected, `Error::NotFound` otherwise.
    pub fn find() -> Ev3Result<Self> {
        let large_motor = LargeMotor::find();
        if let Ok(motor) = large_motor {
            return Ok(TachoMotor {
                inner: TachoMotorInner::LargeMotor { motor },
            });
        }

        let medium_motor = MediumMotor::find();
        if let Ok(motor) = medium_motor {
            return Ok(TachoMotor {
                inner: TachoMotorInner::MediumMotor { motor },
            });
        }

        Err(Ev3Error::InternalError {
            msg: "Could not find a tacho motor at requested port!".to_owned(),
        })
    }

    /// Extract list of connected 'Self'
    pub fn list() -> Ev3Result<Vec<Self>> {
        let large_motor = LargeMotor::list()?;
        let medium_motor = MediumMotor::list()?;

        let mut vec = Vec::with_capacity(large_motor.len() + medium_motor.len());

        vec.extend(large_motor.into_iter().map(|motor| TachoMotor {
            inner: TachoMotorInner::LargeMotor { motor },
        }));

        vec.extend(medium_motor.into_iter().map(|motor| TachoMotor {
            inner: TachoMotorInner::MediumMotor { motor },
        }));

        Ok(vec)
    }

    /// Try to convert this tacho motor to an `LargeMotor`, return `Self` if this fails.
    pub fn into_large_motor(self) -> Result<LargeMotor, TachoMotor> {
        match self.inner {
            TachoMotorInner::LargeMotor { motor } => Ok(motor),
            inner @ TachoMotorInner::MediumMotor { motor: _ } => Err(TachoMotor { inner }),
        }
    }

    /// Try to convert this tacho motor to an `LargeMotor`, return `Self` if this fails.
    pub fn into_medium_motor(self) -> Result<MediumMotor, TachoMotor> {
        match self.inner {
            inner @ TachoMotorInner::LargeMotor { motor: _ } => Err(TachoMotor { inner }),
            TachoMotorInner::MediumMotor { motor } => Ok(motor),
        }
    }

    /// Causes the motor to run until another command is sent.
    pub const COMMAND_RUN_FOREVER: &'static str = "run-forever";

    /// Runs the motor to an absolute position specified by `position_sp`
    /// and then stops the motor using the command specified in `stop_action`.
    pub const COMMAND_RUN_TO_ABS_POS: &'static str = "run-to-abs-pos";

    /// Runs the motor to a position relative to the current position value.
    /// The new position will be current `position` + `position_sp`.
    /// When the new position is reached, the motor will stop using the command specified by `stop_action`.
    pub const COMMAND_RUN_TO_REL_POS: &'static str = "run-to-rel-pos";

    /// Run the motor for the amount of time specified in `time_sp`
    /// and then stops the motor using the command specified by `stop_action`.
    pub const COMMAND_RUN_TIMED: &'static str = "run-timed";

    /// Runs the motor using the duty cycle specified by `duty_cycle_sp`.
    /// Unlike other run commands, changing `duty_cycle_sp` while running will take effect immediately.
    pub const COMMAND_RUN_DIRECT: &'static str = "run-direct";

    /// Stop any of the run commands before they are complete using the command specified by `stop_action`.
    pub const COMMAND_STOP: &'static str = "stop";

    /// Resets all of the motor parameter attributes to their default values.
    /// This will also have the effect of stopping the motor.
    pub const COMMAND_RESET: &'static str = "reset";

    /// A positive duty cycle will cause the motor to rotate clockwise.
    pub const POLARITY_NORMAL: &'static str = "normal";

    /// A positive duty cycle will cause the motor to rotate counter-clockwise.
    pub const POLARITY_INVERSED: &'static str = "inversed";

    /// Power is being sent to the motor.
    pub const STATE_RUNNING: &'static str = "running";

    /// The motor is ramping up or down and has not yet reached a pub constant output level.
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

    /// Returns the number of tacho counts in one rotation of the motor.
    ///
    /// Tacho counts are used by the position and speed attributes,
    /// so you can use this value to convert from rotations or degrees to tacho counts.
    /// (rotation motors only)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::motors::LargeMotor;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Init a tacho motor.
    /// let motor = LargeMotor::find()?;
    ///
    /// // Get position and count_per_rot as f32.
    /// let position = motor.get_position()? as f32;
    /// let count_per_rot = motor.get_count_per_rot()? as f32;
    ///
    /// // Calculate the rotation count.
    /// let rotations = position / count_per_rot;
    ///
    /// println!("The motor did {:.2} rotations", rotations);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_count_per_rot(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_count_per_rot(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_count_per_rot(),
        }
    }

    /// Returns the number of tacho counts in one meter of travel of the motor.
    ///
    /// Tacho counts are used by the position and speed attributes,
    /// so you can use this value to convert from distance to tacho counts.
    /// (linear motors only)
    pub fn get_count_per_m(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_count_per_m(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_count_per_m(),
        }
    }

    /// Returns the number of tacho counts in the full travel of the motor.
    ///
    /// When combined with the count_per_m attribute,
    /// you can use this value to calculate the maximum travel distance of the motor.
    /// (linear motors only)
    pub fn get_full_travel_count(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_full_travel_count(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_full_travel_count(),
        }
    }

    /// Returns the current duty cycle of the motor. Units are percent.
    ///
    /// Values are -100 to 100.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::motors::LargeMotor;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Init a tacho motor.
    /// let motor = LargeMotor::find()?;
    ///
    /// // Set the motor command `run-direct` to start rotation.
    /// motor.run_direct()?;
    ///
    /// // Rotate motor forward and wait 5 seconds.
    /// motor.set_duty_cycle_sp(50)?;
    /// thread::sleep(Duration::from_secs(5));
    ///
    /// assert_eq!(motor.get_duty_cycle()?, 50);
    /// # Ok(())
    /// # }
    pub fn get_duty_cycle(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_duty_cycle(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_duty_cycle(),
        }
    }

    /// Returns the current duty cycle setpoint of the motor.
    ///
    /// Units are in percent.
    /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::motors::LargeMotor;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Init a tacho motor.
    /// let motor = LargeMotor::find()?;
    ///
    /// // Rotate motor forward and wait 5 seconds.
    /// motor.set_duty_cycle_sp(50)?;
    /// thread::sleep(Duration::from_secs(5));
    ///
    /// assert_eq!(motor.get_duty_cycle()?, 50);
    /// # Ok(())
    /// # }
    pub fn get_duty_cycle_sp(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_duty_cycle_sp(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_duty_cycle_sp(),
        }
    }

    /// Sets the duty cycle setpoint of the motor.
    ///
    /// Units are in percent.
    /// Valid values are -100 to 100. A negative value causes the motor to rotate in reverse.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::motors::LargeMotor;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Init a tacho motor.
    /// let motor = LargeMotor::find()?;
    ///
    /// // Set the motor command `run-direct` to start rotation.
    /// motor.run_direct()?;
    ///
    /// // Rotate motor forward and wait 5 seconds.
    /// motor.set_duty_cycle_sp(50)?;
    /// thread::sleep(Duration::from_secs(5));
    ///
    /// // Rotate motor backward and wait 5 seconds.
    /// motor.set_duty_cycle_sp(-50)?;
    /// thread::sleep(Duration::from_secs(5));
    /// # Ok(())
    /// # }
    pub fn set_duty_cycle_sp(&self, duty_cycle: i32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_duty_cycle_sp(duty_cycle),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_duty_cycle_sp(duty_cycle),
        }
    }

    /// Returns the current polarity of the motor.
    pub fn get_polarity(&self) -> Ev3Result<String> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_polarity(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_polarity(),
        }
    }

    /// Sets the polarity of the motor.
    pub fn set_polarity(&self, polarity: &str) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_polarity(polarity),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_polarity(polarity),
        }
    }

    /// Returns the current position of the motor in pulses of the rotary encoder.
    ///
    /// When the motor rotates clockwise, the position will increase.
    /// Likewise, rotating counter-clockwise causes the position to decrease.
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::motors::LargeMotor;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Init a tacho motor.
    /// let motor = LargeMotor::find()?;
    ///
    /// // Get position and count_per_rot as f32.
    /// let position = motor.get_position()? as f32;
    /// let count_per_rot = motor.get_count_per_rot()? as f32;
    ///
    /// // Calculate the rotation count.
    /// let rotations: f32 = position / count_per_rot;
    ///
    /// println!("The motor did {:.2} rotations", rotations);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_position(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_position(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_position(),
        }
    }

    /// Sets the current position of the motor in pulses of the rotary encoder.
    ///
    /// When the motor rotates clockwise, the position will increase.
    /// Likewise, rotating counter-clockwise causes the position to decrease.
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::motors::LargeMotor;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Init a tacho motor.
    /// let motor = LargeMotor::find()?;
    ///
    /// motor.set_position(0)?;
    /// let position = motor.get_position()?;
    ///
    /// // If the motor is not moving, the position value
    /// // should not change between set and get operation.
    /// assert_eq!(position, 0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_position(&self, position: i32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_position(position),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_position(position),
        }
    }

    /// Returns the proportional pub constant for the position PID.
    pub fn get_hold_pid_kp(&self) -> Ev3Result<f32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_hold_pid_kp(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_hold_pid_kp(),
        }
    }

    /// Sets the proportional pub constant for the position PID.
    pub fn set_hold_pid_kp(&self, kp: f32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_hold_pid_kp(kp),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_hold_pid_kp(kp),
        }
    }

    /// Returns the integral pub constant for the position PID.
    pub fn get_hold_pid_ki(&self) -> Ev3Result<f32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_hold_pid_ki(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_hold_pid_ki(),
        }
    }

    /// Sets the integral pub constant for the position PID.
    pub fn set_hold_pid_ki(&self, ki: f32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_hold_pid_ki(ki),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_hold_pid_ki(ki),
        }
    }

    /// Returns the derivative pub constant for the position PID.
    pub fn get_hold_pid_kd(&self) -> Ev3Result<f32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_hold_pid_kd(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_hold_pid_kd(),
        }
    }

    /// Sets the derivative pub constant for the position PID.
    pub fn set_hold_pid_kd(&self, kd: f32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_hold_pid_kd(kd),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_hold_pid_kd(kd),
        }
    }

    /// Returns the maximum value that is accepted by the `speed_sp` attribute.
    ///
    /// This value is the speed of the motor at 9V with no load.
    /// Note: The actual maximum obtainable speed will be less than this
    /// and will depend on battery voltage and mechanical load on the motor.
    pub fn get_max_speed(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_max_speed(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_max_speed(),
        }
    }

    /// Returns the current target position for the `run-to-abs-pos` and `run-to-rel-pos` commands.
    ///
    /// Units are in tacho counts.
    /// You can use the value returned by `counts_per_rot` to convert tacho counts to/from rotations or degrees.
    ///
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer).
    pub fn get_position_sp(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_position_sp(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_position_sp(),
        }
    }

    /// Sets the target position for the `run-to-abs-pos` and `run-to-rel-pos` commands.
    ///
    /// Units are in tacho counts.
    /// You can use the value returned by `counts_per_rot` to convert tacho counts to/from rotations or degrees.
    ///
    /// The range is -2,147,483,648 and +2,147,483,647 tachometer counts (32-bit signed integer).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ev3dev_lang_rust::motors::LargeMotor;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Init a tacho motor.
    /// let motor = LargeMotor::find()?;
    ///
    /// // Save the current position.
    /// let old_position = motor.get_position()?;
    ///
    /// // Rotate by 100 ticks
    /// let position = motor.set_position_sp(100)?;
    /// motor.run_to_rel_pos(None)?;
    ///
    /// // Wait till rotation is finished.
    /// motor.wait_until_not_moving(None);
    ///
    /// // The new position should be 100 ticks larger.
    /// let new_position = motor.get_position()?;
    /// assert_eq!(old_position + 100, new_position);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_position_sp(&self, position_sp: i32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_position_sp(position_sp),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_position_sp(position_sp),
        }
    }

    /// Returns the current motor speed in tacho counts per second.
    ///
    /// Note, this is not necessarily degrees (although it is for LEGO motors).
    /// Use the `count_per_rot` attribute to convert this value to RPM or deg/sec.
    pub fn get_speed(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_speed(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_speed(),
        }
    }

    /// Returns the target speed in tacho counts per second used for all run-* commands except run-direct.
    ///
    /// A negative value causes the motor to rotate in reverse
    /// with the exception of run-to-*-pos commands where the sign is ignored.
    /// Use the `count_per_rot` attribute to convert RPM or deg/sec to tacho counts per second.
    /// Use the `count_per_m` attribute to convert m/s to tacho counts per second.
    pub fn get_speed_sp(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_speed_sp(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_speed_sp(),
        }
    }

    /// Sets the target speed in tacho counts per second used for all run-* commands except run-direct.
    ///
    /// A negative value causes the motor to rotate in reverse
    /// with the exception of run-to-*-pos commands where the sign is ignored.
    /// Use the `count_per_rot` attribute to convert RPM or deg/sec to tacho counts per second.
    /// Use the `count_per_m` attribute to convert m/s to tacho counts per second.
    pub fn set_speed_sp(&self, speed_sp: i32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_speed_sp(speed_sp),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_speed_sp(speed_sp),
        }
    }

    /// Returns the current ramp up setpoint.
    ///
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
    pub fn get_ramp_up_sp(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_ramp_up_sp(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_ramp_up_sp(),
        }
    }

    /// Sets the ramp up setpoint.
    ///
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will increase from 0 to 100% of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and max_speed multiplied by ramp_up_sp. Values must not be negative.
    pub fn set_ramp_up_sp(&self, ramp_up_sp: i32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_ramp_up_sp(ramp_up_sp),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_ramp_up_sp(ramp_up_sp),
        }
    }

    /// Returns the current ramp down setpoint.
    ///
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
    pub fn get_ramp_down_sp(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_ramp_down_sp(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_ramp_down_sp(),
        }
    }

    /// Sets the ramp down setpoint.
    ///
    /// Units are in milliseconds and must be positive. When set to a non-zero value,
    /// the motor speed will decrease from 100% down to 0 of `max_speed` over the span of this setpoint.
    /// The actual ramp time is the ratio of the difference between the speed_sp
    /// and the current speed and 0 multiplied by ramp_down_sp. Values must not be negative.
    pub fn set_ramp_down_sp(&self, ramp_down_sp: i32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_ramp_down_sp(ramp_down_sp),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_ramp_down_sp(ramp_down_sp),
        }
    }

    /// Returns the proportional pub constant for the speed regulation PID.
    pub fn get_speed_pid_kp(&self) -> Ev3Result<f32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_speed_pid_kp(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_speed_pid_kp(),
        }
    }

    /// Sets the proportional pub constant for the speed regulation PID.
    pub fn set_speed_pid_kp(&self, kp: f32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_speed_pid_kp(kp),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_speed_pid_kp(kp),
        }
    }

    /// Returns the integral pub constant for the speed regulation PID.
    pub fn get_speed_pid_ki(&self) -> Ev3Result<f32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_speed_pid_ki(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_speed_pid_ki(),
        }
    }

    /// Sets the integral pub constant for the speed regulation PID.
    pub fn set_speed_pid_ki(&self, ki: f32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_speed_pid_ki(ki),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_speed_pid_ki(ki),
        }
    }

    /// Returns the derivative pub constant for the speed regulation PID.
    pub fn get_speed_pid_kd(&self) -> Ev3Result<f32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_speed_pid_kd(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_speed_pid_kd(),
        }
    }

    /// Sets the derivative pub constant for the speed regulation PID.
    pub fn set_speed_pid_kd(&self, kd: f32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_speed_pid_kd(kd),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_speed_pid_kd(kd),
        }
    }

    /// Returns a list of state flags.
    pub fn get_state(&self) -> Ev3Result<Vec<String>> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_state(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_state(),
        }
    }

    /// Returns the current stop action.
    ///
    /// The value determines the motors behavior when command is set to stop.
    pub fn get_stop_action(&self) -> Ev3Result<String> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_stop_action(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_stop_action(),
        }
    }

    /// Sets the stop action.
    ///
    /// The value determines the motors behavior when command is set to stop.
    pub fn set_stop_action(&self, stop_action: &str) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_stop_action(stop_action),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_stop_action(stop_action),
        }
    }

    /// Returns a list of stop actions supported by the motor controller.
    pub fn get_stop_actions(&self) -> Ev3Result<Vec<String>> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_stop_actions(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_stop_actions(),
        }
    }

    /// Returns the current amount of time the motor will run when using the run-timed command.
    ///
    /// Units are in milliseconds. Values must not be negative.
    pub fn get_time_sp(&self) -> Ev3Result<i32> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.get_time_sp(),
            TachoMotorInner::MediumMotor { ref motor } => motor.get_time_sp(),
        }
    }

    /// Sets the amount of time the motor will run when using the run-timed command.
    ///
    /// Units are in milliseconds. Values must not be negative.
    pub fn set_time_sp(&self, time_sp: i32) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.set_time_sp(time_sp),
            TachoMotorInner::MediumMotor { ref motor } => motor.set_time_sp(time_sp),
        }
    }

    /// Runs the motor using the duty cycle specified by `duty_cycle_sp`.
    ///
    /// Unlike other run commands, changing `duty_cycle_sp` while running will take effect immediately.
    pub fn run_direct(&self) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.run_direct(),
            TachoMotorInner::MediumMotor { ref motor } => motor.run_direct(),
        }
    }

    /// Causes the motor to run until another command is sent.
    pub fn run_forever(&self) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.run_forever(),
            TachoMotorInner::MediumMotor { ref motor } => motor.run_forever(),
        }
    }

    /// Runs the motor to an absolute position specified by `position_sp`
    ///
    /// and then stops the motor using the command specified in `stop_action`.
    pub fn run_to_abs_pos(&self, position_sp: Option<i32>) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.run_to_abs_pos(position_sp),
            TachoMotorInner::MediumMotor { ref motor } => motor.run_to_abs_pos(position_sp),
        }
    }

    /// Runs the motor to a position relative to the current position value.
    ///
    /// The new position will be current `position` + `position_sp`.
    /// When the new position is reached, the motor will stop using the command specified by `stop_action`.
    pub fn run_to_rel_pos(&self, position_sp: Option<i32>) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.run_to_rel_pos(position_sp),
            TachoMotorInner::MediumMotor { ref motor } => motor.run_to_rel_pos(position_sp),
        }
    }

    /// Run the motor for the amount of time specified in `time_sp`
    ///
    /// and then stops the motor using the command specified by `stop_action`.
    pub fn run_timed(&self, time_sp: Option<Duration>) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.run_timed(time_sp),
            TachoMotorInner::MediumMotor { ref motor } => motor.run_timed(time_sp),
        }
    }

    /// Stop any of the run commands before they are complete using the command specified by `stop_action`.
    pub fn stop(&self) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.stop(),
            TachoMotorInner::MediumMotor { ref motor } => motor.stop(),
        }
    }

    /// Resets all of the motor parameter attributes to their default values.
    /// This will also have the effect of stopping the motor.
    pub fn reset(&self) -> Ev3Result<()> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.reset(),
            TachoMotorInner::MediumMotor { ref motor } => motor.reset(),
        }
    }

    /// Power is being sent to the motor.
    pub fn is_running(&self) -> Ev3Result<bool> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.is_running(),
            TachoMotorInner::MediumMotor { ref motor } => motor.is_running(),
        }
    }

    /// The motor is ramping up or down and has not yet reached a pub constant output level.
    pub fn is_ramping(&self) -> Ev3Result<bool> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.is_ramping(),
            TachoMotorInner::MediumMotor { ref motor } => motor.is_ramping(),
        }
    }

    /// The motor is not turning, but rather attempting to hold a fixed position.
    pub fn is_holding(&self) -> Ev3Result<bool> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.is_holding(),
            TachoMotorInner::MediumMotor { ref motor } => motor.is_holding(),
        }
    }

    /// The motor is turning as fast as possible, but cannot reach its `speed_sp`.
    pub fn is_overloaded(&self) -> Ev3Result<bool> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.is_overloaded(),
            TachoMotorInner::MediumMotor { ref motor } => motor.is_overloaded(),
        }
    }

    /// The motor is trying to run but is not turning at all.
    pub fn is_stalled(&self) -> Ev3Result<bool> {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.is_stalled(),
            TachoMotorInner::MediumMotor { ref motor } => motor.is_stalled(),
        }
    }

    /// Wait until condition `cond` returns true or the `timeout` is reached.
    ///
    /// The condition is checked when to the `state` attribute has changed.
    /// If the `timeout` is `None` it will wait an infinite time.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::motors::LargeMotor;
    /// use std::time::Duration;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Init a tacho motor.
    /// let motor = LargeMotor::find()?;
    ///
    /// motor.run_timed(Some(Duration::from_secs(5)))?;
    ///
    /// let cond = || {
    ///     motor.get_state()
    ///         .unwrap_or_else(|_| vec![])
    ///         .iter()
    ///         .all(|s| s != LargeMotor::STATE_RUNNING)
    /// };
    /// motor.wait(cond, None);
    ///
    /// println!("Motor has stopped!");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(target_os = "linux")]
    pub fn wait<F>(&self, cond: F, timeout: Option<Duration>) -> bool
    where
        F: Fn() -> bool,
    {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.wait(cond, timeout),
            TachoMotorInner::MediumMotor { ref motor } => motor.wait(cond, timeout),
        }
    }

    /// Wait while the `state` is in the vector `self.get_state()` or the `timeout` is reached.
    ///
    /// If the `timeout` is `None` it will wait an infinite time.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::motors::LargeMotor;
    /// use std::time::Duration;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Init a tacho motor.
    /// let motor = LargeMotor::find()?;
    ///
    /// motor.run_timed(Some(Duration::from_secs(5)))?;
    ///
    /// motor.wait_while(LargeMotor::STATE_RUNNING, None);
    ///
    /// println!("Motor has stopped!");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(target_os = "linux")]
    pub fn wait_while(&self, state: &str, timeout: Option<Duration>) -> bool {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.wait_while(state, timeout),
            TachoMotorInner::MediumMotor { ref motor } => motor.wait_while(state, timeout),
        }
    }

    /// Wait until the `state` is in the vector `self.get_state()` or the `timeout` is reached.
    ///
    /// If the `timeout` is `None` it will wait an infinite time.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::motors::LargeMotor;
    /// use std::time::Duration;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Init a tacho motor.
    /// let motor = LargeMotor::find()?;
    ///
    /// motor.run_timed(Some(Duration::from_secs(5)))?;
    ///
    /// motor.wait_until(LargeMotor::STATE_RUNNING, None);
    ///
    /// println!("Motor has started!");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(target_os = "linux")]
    pub fn wait_until(&self, state: &str, timeout: Option<Duration>) -> bool {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.wait_until(state, timeout),
            TachoMotorInner::MediumMotor { ref motor } => motor.wait_until(state, timeout),
        }
    }

    /// Wait until the motor is not moving or the timeout is reached.
    ///
    /// This is equal to `wait_while(STATE_RUNNING, timeout)`.
    /// If the `timeout` is `None` it will wait an infinite time.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::motors::LargeMotor;
    /// use std::time::Duration;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Init a tacho motor.
    /// let motor = LargeMotor::find()?;
    ///
    /// motor.run_timed(Some(Duration::from_secs(5)))?;
    ///
    /// motor.wait_until_not_moving(None);
    ///
    /// println!("Motor has stopped!");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(target_os = "linux")]
    pub fn wait_until_not_moving(&self, timeout: Option<Duration>) -> bool {
        match self.inner {
            TachoMotorInner::LargeMotor { ref motor } => motor.wait_until_not_moving(timeout),
            TachoMotorInner::MediumMotor { ref motor } => motor.wait_until_not_moving(timeout),
        }
    }
}

impl From<LargeMotor> for TachoMotor {
    fn from(motor: LargeMotor) -> Self {
        Self {
            inner: TachoMotorInner::LargeMotor { motor },
        }
    }
}

impl From<MediumMotor> for TachoMotor {
    fn from(motor: MediumMotor) -> Self {
        Self {
            inner: TachoMotorInner::MediumMotor { motor },
        }
    }
}

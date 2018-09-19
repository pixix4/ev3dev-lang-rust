use core::Motor;
use core::Device;
use core::MotorPort;
use driver::Driver;
use driver::Attribute;
use driver::AttributeResult;

pub const RUN_FOREVER: &'static str = "run-forever";
pub const RUN_TO_ABS_POS: &'static str = "run-to-abs-pos";
pub const RUN_TO_REL_POS: &'static str = "run-to-rel-pos";
pub const RUN_TIMED: &'static str = "run-timed";
pub const RUN_DIRECT: &'static str = "run-direct";
pub const STOP: &'static str = "stop";
pub const RESET: &'static str = "reset";

pub const POLARITY_NORMAL: &'static str = "normal";
pub const POLARITY: &'static str = "reversed";

pub const STATE_RUNNING: &'static str = "running";
pub const STATE_RAMPING: &'static str = "ramping";
pub const STATE_HOLDING: &'static str = "holding";
pub const STATE_OVERLOADED: &'static str = "overloaded";
pub const STATE_STALLED: &'static str = "stalled";

pub const STOP_ACTION_COAST: &'static str = "coast";
pub const STOP_ACTION_BRAKE: &'static str = "brake";
pub const STOP_ACTION_HOLD: &'static str = "hold";

pub trait TachoMotor: Motor {
    fn get_count_per_rot(&mut self) -> AttributeResult<isize> {
        self.get_attribute("count_per_rot").get_int()
    }
    fn get_full_travel_count(&mut self) -> AttributeResult<isize> {
        self.get_attribute("full_travel_count").get_int()
    }

    fn get_duty_cycle(&mut self) -> AttributeResult<isize> {
        self.get_attribute("duty_cycle").get_int()
    }
    fn get_duty_cycle_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("duty_cycle_sp").get_int()
    }
    fn set_duty_cycle_sp(&mut self, duty_cycle: isize) -> AttributeResult<()> {
        self.get_attribute("duty_cycle_sp").set_int(duty_cycle)
    }

    fn get_polarity(&mut self) -> AttributeResult<String> {
        self.get_attribute("polarity").get_str()
    }
    fn set_polarity(&mut self, polarity: String) -> AttributeResult<()> {
        self.get_attribute("polarity").set_str(polarity)
    }

    fn get_position(&mut self) -> AttributeResult<isize> {
        self.get_attribute("position").get_int()
    }
    fn set_position(&mut self, position: isize) -> AttributeResult<()> {
        self.get_attribute("position").set_int(position)
    }

    fn get_hold_pid_kp(&mut self) -> AttributeResult<f32> {
        self.get_attribute("hold_pid_kp").get_float()
    }
    fn set_hold_pid_kp(&mut self, kp: f32) -> AttributeResult<()> {
        self.get_attribute("hold_pid_kp").set_float(kp)
    }
    fn get_hold_pid_ki(&mut self) -> AttributeResult<f32> {
        self.get_attribute("hold_pid_ki").get_float()
    }
    fn set_hold_pid_ki(&mut self, ki: f32) -> AttributeResult<()> {
        self.get_attribute("hold_pid_ki").set_float(ki)
    }
    fn get_hold_pid_kd(&mut self) -> AttributeResult<f32> {
        self.get_attribute("hold_pid_kd").get_float()
    }
    fn set_hold_pid_kd(&mut self, kd: f32) -> AttributeResult<()> {
        self.get_attribute("hold_pid_kd").set_float(kd)
    }

    fn get_max_speed(&mut self) -> AttributeResult<isize> {
        self.get_attribute("max_speed").get_int()
    }

    fn get_position_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("position_sp").get_int()
    }
    fn set_position_sp(&mut self, position_sp: isize) -> AttributeResult<()> {
        self.get_attribute("position_sp").set_int(position_sp)
    }

    fn get_speed(&mut self) -> AttributeResult<isize> {
        self.get_attribute("speed").get_int()
    }
    fn set_speed(&mut self, speed: isize) -> AttributeResult<()> {
        self.get_attribute("speed").set_int(speed)
    }

    fn get_speed_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("speed_sp").get_int()
    }
    fn set_speed_sp(&mut self, speed_sp: isize) -> AttributeResult<()> {
        self.get_attribute("speed_sp").set_int(speed_sp)
    }

    fn get_ramp_up_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("ramp_up_sp").get_int()
    }
    fn set_ramp_up_sp(&mut self, ramp_up_sp: isize) -> AttributeResult<()> {
        self.get_attribute("ramp_up_sp").set_int(ramp_up_sp)
    }

    fn get_ramp_down_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("ramp_down_sp").get_int()
    }
    fn set_ramp_down_sp(&mut self, ramp_down_sp: isize) -> AttributeResult<()> {
        self.get_attribute("ramp_down_sp").set_int(ramp_down_sp)
    }

    fn get_speed_pid_kp(&mut self) -> AttributeResult<f32> {
        self.get_attribute("speed_pid_kp").get_float()
    }
    fn set_speed_pid_kp(&mut self, kp: f32) -> AttributeResult<()> {
        self.get_attribute("speed_pid_kp").set_float(kp)
    }
    fn get_speed_pid_ki(&mut self) -> AttributeResult<f32> {
        self.get_attribute("speed_pid_ki").get_float()
    }
    fn set_speed_pid_ki(&mut self, ki: f32) -> AttributeResult<()> {
        self.get_attribute("speed_pid_ki").set_float(ki)
    }
    fn get_speed_pid_kd(&mut self) -> AttributeResult<f32> {
        self.get_attribute("speed_pid_kd").get_float()
    }
    fn set_speed_pid_kd(&mut self, kd: f32) -> AttributeResult<()> {
        self.get_attribute("speed_pid_kd").set_float(kd)
    }

    fn get_state(&mut self) -> AttributeResult<Vec<String>> {
        self.get_attribute("state").get_vec()
    }

    fn get_stop_action(&mut self) -> AttributeResult<String> {
        self.get_attribute("stop_action").get_str()
    }
    fn set_stop_action(&mut self, stop_action: String) -> AttributeResult<()> {
        self.get_attribute("stop_action").set_str(stop_action)
    }

    fn get_time_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("time_sp").get_int()
    }
    fn set_time_sp(&mut self, time_sp: isize) -> AttributeResult<()> {
        self.get_attribute("time_sp").set_int(time_sp)
    }

    fn run_direct(&mut self) -> AttributeResult<()> {
        self.set_command(String::from(RUN_DIRECT))
    }

    /*
    fn is_running(&mut self) -> AttributeResult<bool> {
        Ok(self.get_state().iter().find(|&&state| state == STATE_RUNNING).is_some())
    }
    fn is_ramping(&mut self) -> AttributeResult<bool> {
        Ok(self.get_state().iter().find(|&&state| state == STATE_RAMPING).is_some())
    }
    fn is_holding(&mut self) -> AttributeResult<bool> {
        Ok(self.get_state().iter().find(|&&state| state == STATE_HOLDING).is_some())
    }
    fn is_overloaded(&mut self) -> AttributeResult<bool> {
        Ok(self.get_state().iter().find(|&&state| state == STATE_OVERLOADED).is_some())
    }
    fn is_stalled(&mut self) -> AttributeResult<bool> {
        Ok(self.get_state().iter().find(|&&state| state == STATE_STALLED).is_some())
    }
    */
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
    pub fn new(port: MotorPort) -> Option<LargeMotor> {
        if let Some(name) = Driver::find_name_by_port_and_driver("tacho-motor", &port, "lego-ev3-l-motor") {
            return Some(LargeMotor {
                driver: Driver::new(String::from("tacho-motor"), name)
            });
        }

        None
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
    pub fn new(port: MotorPort) -> Option<MediumMotor> {
        if let Some(name) = Driver::find_name_by_port_and_driver("tacho-motor", &port, "lego-ev3-m-motor") {
            return Some(MediumMotor {
                driver: Driver::new(String::from("tacho-motor"), name)
            });
        }

        None
    }
}
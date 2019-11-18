use crate::motors::Motor;
use driver::AttributeResult;

pub const RUN: &str = "run";
pub const FLOAT: &str = "float";

pub const POLARITY_NORMAL: &str = "normal";
pub const POLARITY: &str = "reversed";

pub const STATE_RUNNING: &str = "running";

pub trait ServoMotor: Motor {
    fn get_address(&self) -> AttributeResult<String> {
        self.get_attribute("address").get()
    }
    fn set_command(&self, command: &str) -> AttributeResult<()> {
        self.get_attribute("command").set_str_slice(command)
    }

    fn get_driver_name(&self) -> AttributeResult<String> {
        self.get_attribute("driver_name").get()
    }

    fn get_polarity(&self) -> AttributeResult<String> {
        self.get_attribute("polarity").get()
    }
    fn set_polarity(&self, polarity: &str) -> AttributeResult<()> {
        self.get_attribute("polarity").set_str_slice(polarity)
    }

    fn get_max_pulse_sp(&self) -> AttributeResult<i32> {
        self.get_attribute("max_pulse_sp").get()
    }
    fn set_max_pulse_sp(&self, max_pulse_sp: i32) -> AttributeResult<()> {
        self.get_attribute("max_pulse_sp").set(max_pulse_sp)
    }

    fn get_min_pulse_sp(&self) -> AttributeResult<i32> {
        self.get_attribute("min_pulse_sp").get()
    }
    fn set_min_pulse_sp(&self, min_pulse_sp: i32) -> AttributeResult<()> {
        self.get_attribute("address").set(min_pulse_sp)
    }

    fn get_position_sp(&self) -> AttributeResult<i32> {
        self.get_attribute("position_sp").get()
    }
    fn set_position_sp(&self, position_sp: i32) -> AttributeResult<()> {
        self.get_attribute("position_sp").set(position_sp)
    }

    fn get_rate_sp(&self) -> AttributeResult<i32> {
        self.get_attribute("rate_sp").get()
    }
    fn set_rate_sp(&self, rate_sp: i32) -> AttributeResult<()> {
        self.get_attribute("rate_sp").set(rate_sp)
    }

    fn get_state(&self) -> AttributeResult<Vec<String>> {
        self.get_attribute("state").get_vec()
    }

    fn is_running(&self) -> AttributeResult<bool> {
        Ok(self.get_state()?.iter().any(|state| state == STATE_RUNNING))
    }
}

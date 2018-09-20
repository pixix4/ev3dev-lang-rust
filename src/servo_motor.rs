use core::Motor;
use driver::AttributeResult;

pub const RUN: &'static str = "run";
pub const FLOAT: &'static str = "float";

pub const POLARITY_NORMAL: &'static str = "normal";
pub const POLARITY: &'static str = "reversed";

pub const STATE_RUNNING: &'static str = "running";

trait ServoMotor: Motor {

    fn get_address(&mut self) -> AttributeResult<String> {
        self.get_attribute("address").get_str()
    }
    fn set_command(&mut self, command: String) -> AttributeResult<()> {
        self.get_attribute("command").set_str(command)
    }

    fn get_driver_name(&mut self) -> AttributeResult<String> {
        self.get_attribute("driver_name").get_str()
    }

    fn get_polarity(&mut self) -> AttributeResult<String> {
        self.get_attribute("polarity").get_str()
    }
    fn set_polarity(&mut self, polarity: String) -> AttributeResult<()> {
        self.get_attribute("polarity").set_str(polarity)
    }

    fn get_max_pulse_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("max_pulse_sp").get_int()
    }
    fn set_max_pulse_sp(&mut self, max_pulse_sp: isize) -> AttributeResult<()> {
        self.get_attribute("max_pulse_sp").set_int(max_pulse_sp)
    }

    fn get_min_pulse_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("min_pulse_sp").get_int()
    }
    fn set_min_pulse_sp(&mut self, min_pulse_sp: isize) -> AttributeResult<()> {
        self.get_attribute("address").set_int(min_pulse_sp)
    }

    fn get_position_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("position_sp").get_int()
    }
    fn set_position_sp(&mut self, position_sp: isize) -> AttributeResult<()> {
        self.get_attribute("position_sp").set_int(position_sp)
    }

    fn get_rate_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("rate_sp").get_int()
    }
    fn set_rate_sp(&mut self, rate_sp: isize) -> AttributeResult<()> {
        self.get_attribute("rate_sp").set_int(rate_sp)
    }

    fn get_state(&mut self) -> AttributeResult<Vec<String>> {
        self.get_attribute("state").get_vec()
    }

    /*
    fn is_running(&mut self) -> AttributeResult<bool> {
        Ok(self.get_state().iter().find(|&&state| state == STATE_RUNNING).is_some())
    }
    */
}

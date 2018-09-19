use core::Motor;
use driver::AttributeResult;

const RUN_FOREVER: &'static str = "run-forever";
const RUN_TIMED: &'static str = "run-timed";
const RUN_DIRECT: &'static str = "run-direct";
const STOP: &'static str = "stop";

const POLARITY_NORMAL: &'static str = "normal";
const POLARITY: &'static str = "reversed";

const STATE_RUNNING: &'static str = "running";
const STATE_RAMPING: &'static str = "ramping";

const STOP_ACTION_COAST: &'static str = "coast";
const STOP_ACTION_BRAKE: &'static str = "brake";

trait DcMotor: Motor {

    fn get_address(&mut self) -> AttributeResult<String> {
        self.get_attribute("address").get_str()
    }
    fn set_command(&mut self, command: String) -> AttributeResult<()> {
        self.get_attribute("command").set_str(command)
    }
    fn get_commands(&mut self) -> AttributeResult<Vec<String>> {
        self.get_attribute("commands").get_vec()
    }

    fn get_driver_name(&mut self) -> AttributeResult<String> {
        self.get_attribute("driver_name").get_str()
    }

    fn get_duty_cycle(&mut self) -> AttributeResult<isize> {
        self.get_attribute("duty_cycle").get_int()
    }
    fn get_duty_cycle_sp(&mut self) -> AttributeResult<isize> {
        self.get_attribute("duty_cycle_sp").get_int()
    }
    fn set_duty_cycle_sp(&mut self, duty_cycle_sp: isize) -> AttributeResult<()> {
        self.get_attribute("duty_cycle_sp").set_int(duty_cycle_sp)
    }

    fn get_polarity(&mut self) -> AttributeResult<String> {
        self.get_attribute("polarity").get_str()
    }
    fn set_polarity(&mut self, polarity: String) -> AttributeResult<()> {
        self.get_attribute("polarity").set_str(polarity)
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

    /*
    fn is_running(&mut self) -> AttributeResult<bool> {
        Ok(self.get_state().iter().find(|&&state| state == STATE_RUNNING).is_some())
    }
    fn is_ramping(&mut self) -> AttributeResult<bool> {
        Ok(self.get_state().iter().find(|&&state| state == STATE_RAMPING).is_some())
    }
    */
}
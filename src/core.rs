use driver::Attribute;
use driver::AttributeResult;

pub trait Device {
    fn get_attribute(&mut self, name: &str) -> &Attribute;


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
}

pub trait Motor: Device {}

pub trait Sensor: Device {
    fn get_bin_data(&mut self) -> AttributeResult<String> {
        self.get_attribute("bin_data").get_str()
    }
    fn get_bin_data_format(&mut self) -> AttributeResult<String> {
        self.get_attribute("bin_data_format").get_str()
    }

    fn get_decimals(&mut self) -> AttributeResult<isize> {
        self.get_attribute("decimals").get_int()
    }

    fn get_fw_version(&mut self) -> AttributeResult<String> {
        self.get_attribute("fw_version").get_str()
    }

    fn get_mode(&mut self) -> AttributeResult<String> {
        self.get_attribute("mode").get_str()
    }
    fn set_mode(&mut self, mode: String) -> AttributeResult<()> {
        self.get_attribute("mode").set_str(mode)
    }

    fn get_modes(&mut self) -> AttributeResult<Vec<String>> {
        self.get_attribute("modes").get_vec()
    }

    fn get_num_values(&mut self) -> AttributeResult<isize> {
        self.get_attribute("num_values").get_int()
    }

    fn get_poll_ms(&mut self) -> AttributeResult<isize> {
        self.get_attribute("poll_ms").get_int()
    }
    fn set_poll_ms(&mut self, poll_ms: isize) -> AttributeResult<()> {
        self.get_attribute("poll_ms").set_int(poll_ms)
    }

    fn get_units(&mut self) -> AttributeResult<String> {
        self.get_attribute("units").get_str()
    }

    fn get_value0(&mut self) -> AttributeResult<isize> {
        self.get_attribute("value0").get_int()
    }
    fn get_value1(&mut self) -> AttributeResult<isize> {
        self.get_attribute("value1").get_int()
    }
    fn get_value2(&mut self) -> AttributeResult<isize> {
        self.get_attribute("value2").get_int()
    }
    fn get_value3(&mut self) -> AttributeResult<isize> {
        self.get_attribute("value3").get_int()
    }
    fn get_value4(&mut self) -> AttributeResult<isize> {
        self.get_attribute("value4").get_int()
    }
    fn get_value5(&mut self) -> AttributeResult<isize> {
        self.get_attribute("value5").get_int()
    }
    fn get_value6(&mut self) -> AttributeResult<isize> {
        self.get_attribute("value6").get_int()
    }
    fn get_value7(&mut self) -> AttributeResult<isize> {
        self.get_attribute("value7").get_int()
    }

    fn get_text_value(&mut self) -> AttributeResult<String> {
        self.get_attribute("text_value").get_str()
    }
}

pub trait Port {
    fn address(&self) -> String;
}

pub enum MotorPort {
    OutA,
    OutB,
    OutC,
    OutD,
}

impl Port for MotorPort {
    fn address(&self) -> String {
        match self {
            MotorPort::OutA => String::from("outA"),
            MotorPort::OutB => String::from("outB"),
            MotorPort::OutC => String::from("outC"),
            MotorPort::OutD => String::from("outD"),
        }
    }
}

pub enum SensorPort {
    In1,
    In2,
    In3,
    In4,
}

impl Port for SensorPort {
    fn address(&self) -> String {
        match self {
            SensorPort::In1 => String::from("in1"),
            SensorPort::In2 => String::from("in2"),
            SensorPort::In3 => String::from("in3"),
            SensorPort::In4 => String::from("in4"),
        }
    }
}
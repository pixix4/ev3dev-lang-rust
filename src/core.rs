use driver::Attribute;
use driver::AttributeResult;

pub trait Device {
    fn get_attribute(&mut self, name: &str) -> &Attribute;


    /// Returns the name of the port that the motor is connected to.
    fn get_address(&mut self) -> AttributeResult<String> {
        self.get_attribute("address").get_str()
    }

    /// Sends a command to the device controller.
    fn set_command(&mut self, command: String) -> AttributeResult<()> {
        self.get_attribute("command").set_str(command)
    }

    /// Returns a space separated list of commands that are supported by the device controller.
    fn get_commands(&mut self) -> AttributeResult<Vec<String>> {
        self.get_attribute("commands").get_vec()
    }

    /// Returns the name of the driver that provides this device.
    fn get_driver_name(&mut self) -> AttributeResult<String> {
        self.get_attribute("driver_name").get_str()
    }
}

pub trait Motor: Device {}

pub trait Sensor: Device {

    /// Reading the file will give the unscaled raw values in the `value<N>` attributes.
    /// Use `bin_data_format`, `num_values` and the individual sensor documentation to determine how to interpret the data.
    fn get_bin_data(&mut self) -> AttributeResult<String> {
        self.get_attribute("bin_data").get_str()
    }

    /// Returns the format of the values in `bin_data` for the current mode. Possible values are:
    // * u8: Unsigned 8-bit integer (byte)
    // * s8: Signed 8-bit integer (sbyte)
    // * u16: Unsigned 16-bit integer (ushort)
    // * s16: Signed 16-bit integer (short)
    // * s16_be: Signed 16-bit integer, big endian
    // * s32: Signed 32-bit integer (int)
    // * s32_be: Signed 32-bit integer, big endian
    // * float: IEEE 754 32-bit floating point (float)
    fn get_bin_data_format(&mut self) -> AttributeResult<String> {
        self.get_attribute("bin_data_format").get_str()
    }

    /// Returns the number of decimal places for the values in the `value<N>` attributes of the current mode.
    fn get_decimals(&mut self) -> AttributeResult<isize> {
        self.get_attribute("decimals").get_int()
    }

    /// Returns the firmware version of the sensor if available.
    /// Currently only NXT/I2C sensors support this.
    fn get_fw_version(&mut self) -> AttributeResult<String> {
        self.get_attribute("fw_version").get_str()
    }

    /// Returns the current mode.
    /// See the individual sensor documentation for a description of the modes available for each type of sensor.
    fn get_mode(&mut self) -> AttributeResult<String> {
        self.get_attribute("mode").get_str()
    }

    /// Sets the sensor to that mode.
    /// See the individual sensor documentation for a description of the modes available for each type of sensor.
    fn set_mode(&mut self, mode: String) -> AttributeResult<()> {
        self.get_attribute("mode").set_str(mode)
    }

    /// Returns a list of the valid modes for the sensor.
    fn get_modes(&mut self) -> AttributeResult<Vec<String>> {
        self.get_attribute("modes").get_vec()
    }

    /// Returns the number of `value<N>` attributes that will return a valid value for the current mode.
    fn get_num_values(&mut self) -> AttributeResult<isize> {
        self.get_attribute("num_values").get_int()
    }

    /// Returns the polling period of the sensor in milliseconds.
    /// Returns `-EOPNOTSUPP` if changing polling is not supported.
    /// Note: Setting poll_ms too high can cause the input port autodetection to fail.
    /// If this happens, use the mode attribute of the port to force the port to `nxt-i2c mode`. Values must not be negative.
    fn get_poll_ms(&mut self) -> AttributeResult<isize> {
        self.get_attribute("poll_ms").get_int()
    }

    /// Sets the polling period of the sensor in milliseconds.
    /// Setting to 0 disables polling.
    /// Note: Setting poll_ms too high can cause the input port autodetection to fail.
    /// If this happens, use the mode attribute of the port to force the port to `nxt-i2c mode`. Values must not be negative.
    fn set_poll_ms(&mut self, poll_ms: isize) -> AttributeResult<()> {
        self.get_attribute("poll_ms").set_int(poll_ms)
    }

    /// Returns the units of the measured value for the current mode. May return empty string if units are unknown.
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

    /// Returns a space delimited string representing sensor-specific text values. Returns `-EOPNOTSUPP` if a sensor does not support text values.
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
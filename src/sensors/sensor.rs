//! Common utility functions for sensors.

use crate::{Device, Ev3Result};

/// Common utility functions for sensors.
pub trait Sensor: Device {
    /// Reading the file will give the unscaled raw values in the `value<N>` attributes.
    /// Use `bin_data_format`, `num_values` and the individual sensor documentation to determine how to interpret the data.
    fn get_bin_data(&self) -> Ev3Result<String> {
        self.get_attribute("bin_data").get()
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
    fn get_bin_data_format(&self) -> Ev3Result<String> {
        self.get_attribute("bin_data_format").get()
    }

    /// Returns the number of decimal places for the values in the `value<N>` attributes of the current mode.
    fn get_decimals(&self) -> Ev3Result<i32> {
        self.get_attribute("decimals").get()
    }

    /// Returns the firmware version of the sensor if available.
    /// Currently only NXT/I2C sensors support this.
    fn get_fw_version(&self) -> Ev3Result<String> {
        self.get_attribute("fw_version").get()
    }

    /// Returns the current mode.
    /// See the individual sensor documentation for a description of the modes available for each type of sensor.
    fn get_mode(&self) -> Ev3Result<String> {
        self.get_attribute("mode").get()
    }

    /// Sets the sensor to that mode.
    /// See the individual sensor documentation for a description of the modes available for each type of sensor.
    fn set_mode(&self, mode: &str) -> Ev3Result<()> {
        self.get_attribute("mode").set_str_slice(mode)
    }

    /// Returns a list of the valid modes for the sensor.
    fn get_modes(&self) -> Ev3Result<Vec<String>> {
        self.get_attribute("modes").get_vec()
    }

    /// Returns the number of `value<N>` attributes that will return a valid value for the current mode.
    fn get_num_values(&self) -> Ev3Result<i32> {
        self.get_attribute("num_values").get()
    }

    /// Returns the polling period of the sensor in milliseconds.
    /// Returns `-EOPNOTSUPP` if changing polling is not supported.
    /// Note: Setting poll_ms too high can cause the input port auto detection to fail.
    /// If this happens, use the mode attribute of the port to force the port to `nxt-i2c mode`. Values must not be negative.
    fn get_poll_ms(&self) -> Ev3Result<i32> {
        self.get_attribute("poll_ms").get()
    }

    /// Sets the polling period of the sensor in milliseconds.
    /// Setting to 0 disables polling.
    /// Note: Setting poll_ms too high can cause the input port auto detection to fail.
    /// If this happens, use the mode attribute of the port to force the port to `nxt-i2c mode`. Values must not be negative.
    fn set_poll_ms(&self, poll_ms: i32) -> Ev3Result<()> {
        self.get_attribute("poll_ms").set(poll_ms)
    }

    /// Returns the units of the measured value for the current mode. May return empty string if units are unknown.
    fn get_units(&self) -> Ev3Result<String> {
        self.get_attribute("units").get()
    }

    /// Returns the current `value{index}` value if available.
    fn get_value(&self, index: u8) -> Ev3Result<i32> {
        use crate::Ev3Error;
        match index {
            0 => self.get_value0(),
            1 => self.get_value1(),
            2 => self.get_value2(),
            3 => self.get_value3(),
            4 => self.get_value4(),
            5 => self.get_value5(),
            6 => self.get_value6(),
            7 => self.get_value7(),
            _ => Ev3Result::Err(Ev3Error::InternalError {
                msg: format!("Sensor value index {index} is out of bounds [0, 7]"),
            }),
        }
    }

    /// Returns the current `value0` value if available.
    fn get_value0(&self) -> Ev3Result<i32> {
        self.get_attribute("value0").get()
    }

    /// Returns the current `value1` value if available.
    fn get_value1(&self) -> Ev3Result<i32> {
        self.get_attribute("value1").get()
    }

    /// Returns the current `value2` value if available.
    fn get_value2(&self) -> Ev3Result<i32> {
        self.get_attribute("value2").get()
    }

    /// Returns the current `value3` value if available.
    fn get_value3(&self) -> Ev3Result<i32> {
        self.get_attribute("value3").get()
    }

    /// Returns the current `value4` value if available.
    fn get_value4(&self) -> Ev3Result<i32> {
        self.get_attribute("value4").get()
    }

    /// Returns the current `value5` value if available.
    fn get_value5(&self) -> Ev3Result<i32> {
        self.get_attribute("value5").get()
    }

    /// Returns the current `value6` value if available.
    fn get_value6(&self) -> Ev3Result<i32> {
        self.get_attribute("value6").get()
    }

    /// Returns the current `value7` value if available.
    fn get_value7(&self) -> Ev3Result<i32> {
        self.get_attribute("value7").get()
    }

    /// Returns a space delimited string representing sensor-specific text values. Returns `-EOPNOTSUPP` if a sensor does not support text values.
    fn get_text_value(&self) -> Ev3Result<String> {
        self.get_attribute("text_value").get()
    }
}

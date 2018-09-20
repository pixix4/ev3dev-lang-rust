use core::Sensor;
use core::Device;
use core::SensorPort;
use driver::Driver;
use driver::Attribute;
use driver::AttributeResult;

/// Angle
pub const MODE_GYRO_ANG: &'static str = "GYRO-ANG";

/// Rotational Speed
pub const MODE_GYRO_RATE: &'static str = "GYRO-RATE";

/// Raw sensor value ???
pub const MODE_GYRO_FAS: &'static str = "GYRO-FAS";

/// Angle and Rotational Speed
pub const MODE_GYRO_G_AND_A: &'static str = "GYRO-G&A";

/// Calibration ???
pub const MODE_GYRO_CAL: &'static str = "GYRO-CAL";

pub struct GyroSensor {
    driver: Driver
}

impl Sensor for GyroSensor {}

impl Device for GyroSensor {
    fn get_attribute(&mut self, name: &str) -> &Attribute {
        self.driver.get_attribute(name)
    }
}

impl GyroSensor {
    
    /// Try to get a `GyroSensor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: SensorPort) -> Option<GyroSensor> {
        if let Some(name) = Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-gyro") {
            return Some(GyroSensor {
                driver: Driver::new(String::from("lego-sensor"), name)
            });
        }

        None
    }

    /// Try to find a `GyroSensor`. Only returns a sensor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Option<GyroSensor> {
        if let Some(name) = Driver::find_name_by_driver("lego-sensor", "lego-ev3-gyro") {
            return Some(GyroSensor {
                driver: Driver::new(String::from("lego-sensor"), name)
            });
        }

        None
    }

    pub fn set_mode_col_ang(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_GYRO_ANG))
    }

    pub fn set_mode_col_rate(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_GYRO_RATE))
    }

    pub fn set_mode_col_fas(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_GYRO_FAS))
    }

    pub fn set_mode_gyro_g_and_a(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_GYRO_G_AND_A))
    }

    pub fn set_mode_gyro_cal(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_GYRO_CAL))
    }
}
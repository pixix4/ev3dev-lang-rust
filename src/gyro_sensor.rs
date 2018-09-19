use core::Sensor;
use core::Device;
use core::SensorPort;
use driver::Driver;
use driver::Attribute;
use driver::AttributeResult;

pub const MODE_GYRO_ANG: &'static str = "GYRO-ANG";
pub const MODE_GYRO_RATE: &'static str = "GYRO-RATE";
pub const MODE_GYRO_FAS: &'static str = "GYRO-FAS";
pub const MODE_GYRO_G_AND_A: &'static str = "GYRO-G&A";
pub const MODE_GYRO_CAL: &'static str = "GYRO-CAL";

pub struct ColorSensor {
    driver: Driver
}

impl Sensor for ColorSensor {}

impl Device for ColorSensor {
    fn get_attribute(&mut self, name: &str) -> &Attribute {
        self.driver.get_attribute(name)
    }
}

impl ColorSensor {
    pub fn new(port: SensorPort) -> Option<ColorSensor> {
        if let Some(name) = Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-gyro") {
            return Some(ColorSensor {
                driver: Driver::new(String::from("lego-sensor"), name)
            });
        }

        None
    }
    pub fn find() -> Option<ColorSensor> {
        if let Some(name) = Driver::find_name_by_driver("lego-sensor", "lego-ev3-gyro") {
            return Some(ColorSensor {
                driver: Driver::new(String::from("lego-sensor"), name)
            });
        }

        None
    }

    fn set_mode_col_ang(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_GYRO_ANG))
    }

    fn set_mode_col_rate(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_GYRO_RATE))
    }

    fn set_mode_col_fas(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_GYRO_FAS))
    }

    fn set_mode_gyro_g_and_a(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_GYRO_G_AND_A))
    }

    fn set_mode_gyro_cal(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_GYRO_CAL))
    }

    fn get_red(&mut self) -> AttributeResult<isize> {
        self.get_value0()
    }
    fn get_green(&mut self) -> AttributeResult<isize> {
        self.get_value1()
    }
    fn get_blue(&mut self) -> AttributeResult<isize> {
        self.get_value2()
    }
}
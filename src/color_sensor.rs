use core::Sensor;
use core::Device;
use core::SensorPort;
use driver::Driver;
use driver::Attribute;
use driver::AttributeResult;

pub const MODE_COL_REFLECT: &'static str = "COL-REFLECT";
pub const MODE_COL_AMBIENT: &'static str = "COL-AMBIENT";
pub const MODE_COL_COLOR: &'static str = "COL-COLOR";
pub const MODE_REF_RAW: &'static str = "REF-RAW";
pub const MODE_RGB_RAW: &'static str = "RGB-RAW";
pub const MODE_COL_CAL: &'static str = "COL-CAL";

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
        if let Some(name) = Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-color") {
            return Some(ColorSensor {
                driver: Driver::new(String::from("lego-sensor"), name)
            });
        }

        None
    }
    pub fn find() -> Option<ColorSensor> {
        if let Some(name) = Driver::find_name_by_driver("lego-sensor", "lego-ev3-color") {
            return Some(ColorSensor {
                driver: Driver::new(String::from("lego-sensor"), name)
            });
        }

        None
    }

    pub fn set_mode_col_reflect(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_COL_REFLECT))
    }

    pub fn set_mode_col_ambient(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_COL_AMBIENT))
    }

    pub fn set_mode_col_color(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_COL_COLOR))
    }

    pub fn set_mode_ref_raw(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_REF_RAW))
    }

    pub fn set_mode_rgb_raw(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_RGB_RAW))
    }

    pub fn set_mode_col_cal(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_COL_CAL))
    }

    pub fn get_red(&mut self) -> AttributeResult<isize> {
        self.get_value0()
    }
    pub fn get_green(&mut self) -> AttributeResult<isize> {
        self.get_value1()
    }
    pub fn get_blue(&mut self) -> AttributeResult<isize> {
        self.get_value2()
    }

    pub fn get_rgb(&mut self) -> AttributeResult<(isize, isize, isize)> {
        let red = self.get_red()?;
        let green = self.get_green()?;
        let blue = self.get_blue()?;

        return Ok((red, green, blue));
    }
}
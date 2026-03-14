//! Collection of port utils

use crate::{Driver, Ev3Error, Ev3Result};

/// EV3 ports
pub trait Port {
    /// Returns the name of the port.
    fn address(&self) -> String;

    /// Get the port device for this port
    /// # Examples
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::{Ev3Result, Port};
    /// use ev3dev_lang_rust::sensors::{ColorSensor, SensorPort};
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// fn init_color_sensor(port: SensorPort) -> Ev3Result<ColorSensor> {
    ///     let lego_port = port.get_lego_port()?;
    ///     lego_port.set_mode("ev3-uart")?;
    ///     lego_port.set_device("lego-ev3-color")?;
    ///
    ///     thread::sleep(Duration::from_millis(100));
    ///
    ///     ColorSensor::get(port)
    /// }
    ///
    /// # fn main() -> Ev3Result<()> {
    /// let color_sensor = init_color_sensor(SensorPort::In1)?;
    /// # Ok(())
    /// # }
    fn get_lego_port(&self) -> Ev3Result<LegoPort>;
}

/// Lego ports
#[derive(Debug, Clone)]
pub struct LegoPort {
    driver: Driver,
}

impl LegoPort {
    fn new(driver: Driver) -> Self {
        Self { driver }
    }

    fn map_error(e: Ev3Error) -> Ev3Error {
        match e {
            e @ Ev3Error::InternalError { .. } => e,
            Ev3Error::NotConnected { device: _, port } => Ev3Error::NotConnected {
                device: "LegoPort".to_owned(),
                port,
            },
            Ev3Error::MultipleMatches { device: _, ports } => Ev3Error::MultipleMatches {
                device: "LegoPort".to_owned(),
                ports,
            },
        }
    }

    fn driver_names() -> Vec<&'static str> {
        vec![
            "ev3-input-port",
            "ev3-output-port",
            "modbrick-in-port",
            "modbrick-out-port",
        ]
    }

    /// Try to get a `Self` on the given port. Returns `None` if port is not used or another device is connected.
    /// Get the port device for this port
    /// # Examples
    ///
    /// ```no_run
    /// use ev3dev_lang_rust::{Ev3Result, LegoPort};
    /// use ev3dev_lang_rust::sensors::{ColorSensor, SensorPort};
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// fn init_color_sensor(port: SensorPort) -> Ev3Result<ColorSensor> {
    ///     let lego_port = LegoPort::get(port)?;
    ///     lego_port.set_mode("ev3-uart")?;
    ///     lego_port.set_device("lego-ev3-color")?;
    ///
    ///     thread::sleep(Duration::from_millis(100));
    ///
    ///     ColorSensor::get(port)
    /// }
    ///
    /// # fn main() -> Ev3Result<()> {
    /// let color_sensor = init_color_sensor(SensorPort::In1)?;
    /// # Ok(())
    /// # }
    pub fn get<T: Port>(port: T) -> Ev3Result<Self> {
        let driver_name_vec = Self::driver_names();

        let name = Driver::find_name_by_port_and_driver("lego-port", &port, &driver_name_vec)
            .map_err(Self::map_error)?;

        Ok(Self::new(Driver::new("lego-port", &name)))
    }

    /// Try to find a `Self`. Only returns a device if their is exactly one connected, `Error::NotFound` otherwise.
    pub fn find() -> Ev3Result<Self> {
        let driver_name_vec = Self::driver_names();

        let name =
            Driver::find_name_by_driver("lego-port", &driver_name_vec).map_err(Self::map_error)?;

        Ok(Self::new(Driver::new("lego-port", &name)))
    }

    /// Extract list of connected 'Self'
    pub fn list() -> Ev3Result<Vec<Self>> {
        let driver_name_vec = Self::driver_names();

        Ok(Driver::find_names_by_driver("lego-port", &driver_name_vec)?
            .iter()
            .map(|name| Self::new(Driver::new("lego-port", name)))
            .collect())
    }

    /// Returns the name of the port that the device is connected to.
    pub fn get_address(&self) -> Ev3Result<String> {
        self.driver.get_attribute("address").get()
    }

    /// Returns the name of the driver that provides this device.
    pub fn get_driver_name(&self) -> Ev3Result<String> {
        self.driver.get_attribute("driver_name").get()
    }

    /// Returns the currently selected mode.
    pub fn get_mode(&self) -> Ev3Result<String> {
        self.driver.get_attribute("mode").get()
    }

    /// Sets the currently selected mode.
    /// Generally speaking when the mode changes any sensor or motor devices
    /// associated with the port will be removed new ones loaded, however this
    /// this will depend on the individual driver implementing this class.
    pub fn set_mode(&self, mode: &str) -> Ev3Result<()> {
        self.driver.get_attribute("mode").set_str_slice(mode)
    }

    /// Returns a list of the available modes of the port.
    pub fn get_modes(&self) -> Ev3Result<Vec<String>> {
        self.driver.get_attribute("modes").get_vec()
    }

    /// For modes that support it, writing the name of a driver will cause a new
    /// device to be registered for that driver and attached to this port. For
    /// example, since NXT/Analog sensors cannot be auto-detected, you must use
    /// this attribute to load the correct driver. Returns -EOPNOTSUPP if setting a
    /// device is not supported.
    pub fn set_device(&self, mode: &str) -> Ev3Result<()> {
        self.driver.get_attribute("set_device").set_str_slice(mode)
    }

    /// In most cases, reading status will return the same value as `mode`. In
    /// cases where there is an `auto` mode additional values may be returned,
    /// such as `no-device` or `error`. See individual port driver documentation
    /// for the full list of possible values.
    pub fn get_status(&self) -> Ev3Result<String> {
        self.driver.get_attribute("status").get()
    }
}

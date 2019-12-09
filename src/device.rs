use crate::{Attribute, Ev3Result};

/// The ev3dev device base trait
pub trait Device {
    /// Returns the attribute wrapper for an attribute name.
    fn get_attribute(&self, name: &str) -> Attribute;

    /// Returns the name of the port that the motor is connected to.
    fn get_address(&self) -> Ev3Result<String> {
        self.get_attribute("address").get()
    }

    /// Sends a command to the device controller.
    fn set_command(&self, command: &str) -> Ev3Result<()> {
        self.get_attribute("command").set_str_slice(command)
    }

    /// Returns a space separated list of commands that are supported by the device controller.
    fn get_commands(&self) -> Ev3Result<Vec<String>> {
        self.get_attribute("commands").get_vec()
    }

    /// Returns the name of the driver that provides this device.
    fn get_driver_name(&self) -> Ev3Result<String> {
        self.get_attribute("driver_name").get()
    }
}

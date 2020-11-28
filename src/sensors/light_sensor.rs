//! LEGO EV3 light sensor.

use std::cell::Cell;
use super::SensorPort;
use crate::{Attribute, Device, Driver, Ev3Error, Ev3Result};

/// LEGO EV3 light sensor.
#[derive(Debug, Clone, Device)]
pub struct LightSensor {
    driver: Driver,
    reflect_scale: Cell<Option<f32>>,
    ambient_scale: Cell<Option<f32>>,
}

impl LightSensor {

    fn new(driver: Driver) -> Self {
        Self {
            driver,
            reflect_scale: Cell::new(None),
            ambient_scale: Cell::new(None),
        }
    }

    findable!(
        "lego-sensor",
        "lego-nxt-light",
        SensorPort,
        "LightSensor",
        "in"
    );

    /// Reflected light. LED on
    pub const MODE_REFLECT: &'static str = "REFLECT";

    /// Ambient light. LED off
    pub const MODE_AMBIENT: &'static str = "AMBIENT";

    sensor!();

    /// Reflected light. LED on
    pub fn set_mode_reflect(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_REFLECT)
    }

    /// Ambient light. LED off
    pub fn set_mode_ambient(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_AMBIENT)
    }

    /// A measurement of the light intensity, unscaled.
    pub fn get_light_intensity(&self) -> Ev3Result<i32> {
        self.get_value0()
    }

    /// A measurement of the reflected light intensity, as a percentage.
    pub fn get_reflected_light_intensity(&self) -> Ev3Result<f32> {
        let scale_field = self.reflect_scale.get();
        let scale = match scale_field {
            Some(s) => s,
            None => {
                let decimals = self.get_decimals()?;
                let s = 10f32.powi(-decimals);
                self.reflect_scale.set(Some(s));
                s
            }
        };

        Ok((self.get_value0()? as f32) * scale)
    }

    /// A measurement of the ambient light intensity, as a percentage.
    pub fn get_ambient_light_intensity(&self) -> Ev3Result<f32> {
        let scale_field = self.ambient_scale.get();
        let scale = match scale_field {
            Some(s) => s,
            None => {
                let decimals = self.get_decimals()?;
                let s = 10f32.powi(-decimals);
                self.ambient_scale.set(Some(s));
                s
            }
        };

        Ok((self.get_value0()? as f32) * scale)
    }
}

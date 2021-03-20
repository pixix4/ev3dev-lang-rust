//! HiTechnic EV3 / NXT Infrared Sensor. (<https://www.generationrobots.com/de/401172-nxt-irseeker-v2-infrarot-sensor-f%C3%BCr-nxt-und-ev3-mindstorms-.html>)

use super::{Sensor, SensorPort};
use crate::{sensor_mode, Attribute, Device, Driver, Ev3Error, Ev3Result};

/// HiTechnic EV3 / NXT Infrared Sensor.
#[derive(Debug, Clone, Device, Sensor)]
pub struct IrSeekerSensor {
    driver: Driver,
}

impl IrSeekerSensor {
    fn new(driver: Driver) -> Self {
        Self { driver }
    }

    findable!(
        "lego-sensor",
        ["ht-nxt-ir-seek-v2"],
        SensorPort,
        "IrSeeker",
        "in"
    );

    sensor_mode!(
        "AC",
        MODE_AC,
        "Sensor mode alternating current -> filters the infrared signal of the hitechnic ball -> only shows direction",
        set_mode_ac,
        is_mode_ac
    );
    sensor_mode!(
        "DC",
        MODE_DC,
        "Sensor mode direct current -> reacts on all infrared signals, sun infrared signal included -> only shows direction",
        set_mode_dc,
        is_mode_dc
    );
    sensor_mode!(
        "AC-ALL",
        MODE_AC_ALL,
        "Sensor mode alternating current -> shows direction (value0) and values of each of the five sensors",
        set_mode_ac_all,
        is_mode_ac_all
    );
    sensor_mode!(
        "DC-ALL",
        MODE_DC_ALL,
        "Sensor mode direct current -> shows direction (value0) and values of each of the five sensors",
        set_mode_dc_all,
        is_mode_dc_all
    );

    /// gets direction of incoming ir light (calculated by the sensor)
    pub fn get_ir_direction(&self) -> Ev3Result<i32> {
        self.get_value0()
    }

    /// gets the values of the five sensors of the HiTechnic IR Seeker (only works if dc_all or ac_all mode is activated)
    pub fn get_raw_values(&self) -> Ev3Result<[i32; 5]> {
        let val1 = self.get_value1()?;
        let val2 = self.get_value2()?;
        let val3 = self.get_value3()?;
        let val4 = self.get_value4()?;
        let val5 = self.get_value5()?;
        Ok([val1, val2, val3, val4, val5])
    }
}

//! # Container module for sensor types

mod sensor;
pub use self::sensor::Sensor;

mod color_sensor;
pub use self::color_sensor::ColorSensor;

mod hi_technic_color_sensor;
pub use self::hi_technic_color_sensor::HiTechnicColorSensor;

mod ir_seeker_sensor;
pub use self::ir_seeker_sensor::IrSeekerSensor;

mod compass_sensor;
pub use self::compass_sensor::CompassSensor;

mod light_sensor;
pub use self::light_sensor::LightSensor;

mod gyro_sensor;
pub use self::gyro_sensor::GyroSensor;

mod infrared_sensor;
pub use self::infrared_sensor::BeaconSeeker;
pub use self::infrared_sensor::InfraredSensor;
pub use self::infrared_sensor::RemoteControl;

mod touch_sensor;
pub use self::touch_sensor::TouchSensor;

mod ultrasonic_sensor;
pub use self::ultrasonic_sensor::UltrasonicSensor;

use crate::{port_constants, Port};

/// EV3 ports `in1` to `in4`
#[derive(Debug, Copy, Clone)]
pub enum SensorPort {
    /// EV3 `in1` port
    In1,
    /// EV3 `in2` port
    In2,
    /// EV3 `in3` port
    In3,
    /// EV3 `in4` port
    In4,
}

impl SensorPort {
    /// Try to format a device name path to a  port name.
    pub fn format_name(name: &str) -> String {
        match name {
            "sensor0" => SensorPort::In1.address(),
            "sensor1" => SensorPort::In2.address(),
            "sensor2" => SensorPort::In3.address(),
            "sensor3" => SensorPort::In4.address(),
            _ => name.to_owned(),
        }
    }
}

impl Port for SensorPort {
    fn address(&self) -> String {
        match self {
            SensorPort::In1 => port_constants::INPUT_1.to_owned(),
            SensorPort::In2 => port_constants::INPUT_2.to_owned(),
            SensorPort::In3 => port_constants::INPUT_3.to_owned(),
            SensorPort::In4 => port_constants::INPUT_4.to_owned(),
        }
    }
}

#[macro_export]
/// Add a sensor mode constant with getter and setter
macro_rules! sensor_mode {
    ($value:expr, $const_name:ident, $docstring:expr, $setter:ident, $getter:ident) => {
        #[doc = $docstring]
        pub const $const_name: &'static str = $value;

        #[doc = $docstring]
        pub fn $setter(&self) -> Ev3Result<()> {
            self.set_mode(Self::$const_name)
        }

        #[doc = $docstring]
        pub fn $getter(&self) -> Ev3Result<bool> {
            Ok(self.get_mode()? == Self::$const_name)
        }
    };
}

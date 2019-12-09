use crate::{Device, Ev3Result, Port};

/// Helper trait to create a new `Device` instance.
///
/// Can be automatically derived. Therefore are 3 parameters required:
/// * `class_name: &str`
/// * `driver_name: &str`
/// * `port: dyn ev3dev_lang_rust::Motor`
///
/// # Example:
///
/// #[derive(Debug, Clone, Device, Findable, Motor, TachoMotor)]
/// #[class_name = "tacho-motor"]
/// #[driver_name = "lego-ev3-l-motor"]
/// #[port = "crate::motors::MotorPort"]
/// pub struct LargeMotor {
///     driver: Driver,
/// }
pub trait Findable<PortType>
where
    Self: std::marker::Sized,
    Self: Device,
    PortType: Port,
{
    /// Extract list of connected 'Self'
    fn list() -> Ev3Result<Vec<Self>>;

    /// Try to get a `Self` on the given port. Returns `None` if port is not used or another device is connected.
    fn get(port: PortType) -> Ev3Result<Self>;

    /// Try to find a `Self`. Only returns a motor if their is exactly one connected, `Error::NotFound` otherwise.
    fn find() -> Ev3Result<Self>;
}

use crate::{Ev3Result, Port};

pub trait Findable<PortType>
where
    Self: std::marker::Sized,
    PortType: Port,
{
    /// Extract list of connected 'Self'
    fn list() -> Ev3Result<Vec<Self>>;

    /// Try to get a `Self` on the given port. Returns `None` if port is not used or another device is connected.
    fn get(port: PortType) -> Ev3Result<Self>;

    /// Try to find a `Self`. Only returns a motor if their is exactly one connected, `Error::NotFound` otherwise.
    fn find() -> Ev3Result<Self>;
}

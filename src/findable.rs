/// Helper to create a new `Device` instance.
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
#[macro_export]
macro_rules! findable {
    ($class_name:expr, $driver_name:expr, $port: ty) => {
        /// Try to get a `Self` on the given port. Returns `None` if port is not used or another device is connected.
        pub fn get(port: $port) -> Ev3Result<Self> {
            let name = Driver::find_name_by_port_and_driver($class_name, &port, $driver_name)?;

            Ok(Self {
                driver: Driver::new($class_name, &name),
            })
        }

        /// Try to find a `Self`. Only returns a motor if their is exactly one connected, `Error::NotFound` otherwise.
        pub fn find() -> Ev3Result<Self> {
            let name = Driver::find_name_by_driver($class_name, $driver_name)?;

            Ok(Self {
                driver: Driver::new($class_name, &name),
            })
        }

        /// Extract list of connected 'Self'
        pub fn list() -> Ev3Result<Vec<Self>> {
            Ok(Driver::find_names_by_driver($class_name, $driver_name)?
                .iter()
                .map(|name| Self {
                    driver: Driver::new($class_name, name),
                })
                .collect())
        }
    };
}

/// Helper to create a new `Device` instance.
///
/// Generates `get()`, `find()` and `list()` methods. Therefore are 5 parameters required:
/// * `class_name: &str`
/// * `driver_name: &str`
/// * `port: dyn ev3dev_lang_rust::Motor`
/// * `debug_name: &str`
/// * `port_prefix: &str`
///
/// # Example:
///
/// ```ignore
/// #[derive(Debug, Clone, Device)]
/// pub struct LargeMotor {
///     driver: Driver,
/// }
///
/// impl LargeMotor {
///     findable!("tacho-motor", "lego-ev3-l-motor", MotorPort), "LargeMotor", "out";
///     tacho_motor!();
/// }
/// ```
#[macro_export]
macro_rules! findable {
    ($class_name:expr, [$( $driver_name:expr ),*], $port: ty, $debug_name:expr, $port_prefix:expr) => {
        fn map_error(e: Ev3Error) -> Ev3Error {
            match e {
                e @ Ev3Error::InternalError { .. } => e,
                Ev3Error::NotConnected { device: _, port } => Ev3Error::NotConnected {
                    device: $debug_name.to_owned(),
                    port,
                },
                Ev3Error::MultipleMatches { device: _, ports } => Ev3Error::MultipleMatches {
                    device: $debug_name.to_owned(),
                    ports: ports
                        .iter()
                        .map(|item| <$port>::format_name(item))
                        .collect(),
                },
            }
        }

        /// Try to get a `Self` on the given port. Returns `None` if port is not used or another device is connected.
        #[allow(clippy::vec_init_then_push)]
        pub fn get(port: $port) -> Ev3Result<Self> {
            let mut driver_name_vec = Vec::new();
            $(
                driver_name_vec.push($driver_name);
            )*

            let name = Driver::find_name_by_port_and_driver($class_name, &port, &driver_name_vec)
                .map_err(Self::map_error)?;

            Ok(Self::new(Driver::new($class_name, &name)))
        }

        /// Try to find a `Self`. Only returns a motor if their is exactly one connected, `Error::NotFound` otherwise.
        #[allow(clippy::vec_init_then_push)]
        pub fn find() -> Ev3Result<Self> {
            let mut driver_name_vec = Vec::new();
            $(
                driver_name_vec.push($driver_name);
            )*

            let name =
                Driver::find_name_by_driver($class_name, &driver_name_vec).map_err(Self::map_error)?;

            Ok(Self::new(Driver::new($class_name, &name)))
        }

        /// Extract list of connected 'Self'
        #[allow(clippy::vec_init_then_push)]
        pub fn list() -> Ev3Result<Vec<Self>> {
            let mut driver_name_vec = Vec::new();
            $(
                driver_name_vec.push($driver_name);
            )*

            Ok(Driver::find_names_by_driver($class_name, &driver_name_vec)?
                .iter()
                .map(|name| Self::new(Driver::new($class_name, &name)))
                .collect())
        }
    };
}

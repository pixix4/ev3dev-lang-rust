//! Utility things.

/// Helper `Result` type for easy access.
pub type Ev3Result<T> = Result<T, Ev3Error>;

/// Custom error type for internal errors.
#[derive(Debug)]
pub enum Ev3Error {
    /// Internal error with error `msg`.
    InternalError {
        /// Original error message.
        msg: String,
    },
    /// No matching device found.
    NotFound,
    /// More than one matching device found.
    MultipleMatches,
}
impl From<std::io::Error> for Ev3Error {
    fn from(err: std::io::Error) -> Self {
        Ev3Error::InternalError {
            msg: format!("{}", err),
        }
    }
}

impl From<std::string::FromUtf8Error> for Ev3Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Ev3Error::InternalError {
            msg: format!("{}", err),
        }
    }
}

impl From<std::num::ParseIntError> for Ev3Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Ev3Error::InternalError {
            msg: format!("{}", err),
        }
    }
}

/// EV3 ports
pub trait Port {
    /// Returns the name of the port.
    fn address(&self) -> String;
}

/// Helper trait to convert an option to an error.
/// Polyfill for the `Try` trait until it is stable.
pub trait OrErr<T> {
    /// Consumes the `Option<T>` and returns an `Ev3Result<T>`.
    fn or_err(self) -> Ev3Result<T>;
}

impl<T> OrErr<T> for Option<T> {
    fn or_err(self) -> Ev3Result<T> {
        self.ok_or(Ev3Error::InternalError {
            msg: "Cannot unwrap option".to_owned(),
        })
    }
}

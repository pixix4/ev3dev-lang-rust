use std::error::Error;

pub type Ev3Result<T> = Result<T, Ev3Error>;

#[derive(Debug)]
pub enum Ev3Error {
    InternalError { msg: String },
    NotFound,
    MultipleMatches,
}
impl From<std::io::Error> for Ev3Error {
    fn from(err: std::io::Error) -> Ev3Error {
        Ev3Error::InternalError {
            msg: err.description().to_owned(),
        }
    }
}

pub trait Port {
    fn address(&self) -> String;
}

pub trait OrErr<T> {
    fn or_err(self) -> Ev3Result<T>;
}

impl<T> OrErr<T> for Option<T> {
    fn or_err(self) -> Ev3Result<T> {
        self.ok_or(Ev3Error::InternalError {
            msg: "Cannot unwrap option".to_owned(),
        })
    }
}

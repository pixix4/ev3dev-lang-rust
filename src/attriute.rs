//! A wrapper to a attribute file in the `/sys/class/` directory.
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::io::{AsRawFd, RawFd};
use std::string::String;
use std::sync::{Arc, Mutex};

use crate::{Ev3Error, Ev3Result};

/// The root driver path `/sys/class/`.
const ROOT_PATH: &str = "/sys/class/";

/// A wrapper to a attribute file in the `/sys/class/` directory.
#[derive(Debug, Clone)]
pub struct Attribute {
    file: Arc<Mutex<File>>,
}

impl Attribute {
    /// Create a new `Attribute` instance that wrappes
    /// the file `/sys/class/{class_name}/{name}{attribute_name}`.
    pub fn new(class_name: &str, name: &str, attribute_name: &str) -> Ev3Result<Attribute> {
        let filename = format!("{}{}/{}/{}", ROOT_PATH, class_name, name, attribute_name);

        let stat = fs::metadata(&filename)?;

        let mode = stat.permissions().mode();

        let readable = mode & 256 == 256;
        let writeable = mode & 128 == 128;

        let file = OpenOptions::new()
            .read(readable)
            .write(writeable)
            .open(&filename)?;

        Ok(Attribute {
            file: Arc::new(Mutex::new(file)),
        })
    }

    /// Returns the current value of the wrapped file.
    fn get_str(&self) -> Ev3Result<String> {
        let mut value = String::new();
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(0))?;
        file.read_to_string(&mut value)?;
        Ok(value.trim_end().to_owned())
    }

    /// Sets the value of the wrapped file.
    /// Returns a `Ev3Result::InternalError` if the file is not writable.
    fn set_str(&self, value: &str) -> Ev3Result<()> {
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(0))?;
        file.write_all(value.as_bytes())?;
        Ok(())
    }

    /// Returns the current value of the wrapped file.
    /// The value is parsed to the type `T`.
    /// Returns a `Ev3Result::InternalError` if the current value is not parsable to type `T`.
    pub fn get<T>(&self) -> Ev3Result<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: Error,
    {
        let value = self.get_str()?;
        match value.parse::<T>() {
            Ok(value) => Ok(value),
            Err(e) => Err(Ev3Error::InternalError {
                msg: format!("{}", e),
            }),
        }
    }

    /// Sets the value of the wrapped file.
    /// The value is parsed from the type `T`.
    /// Returns a `Ev3Result::InternalError` if the file is not writable.
    pub fn set<T>(&self, value: T) -> Ev3Result<()>
    where
        T: std::string::ToString,
    {
        self.set_str(&value.to_string())
    }

    #[inline]
    /// Sets the value of the wrapped file.
    /// This function skips the string parsing of the `self.set<T>()` function.
    /// Returns a `Ev3Result::InternalError` if the file is not writable.
    pub fn set_str_slice(&self, value: &str) -> Ev3Result<()> {
        self.set_str(value)
    }

    /// Returns a string vector representation of the wrapped file.
    /// The file value is splitet at whitespaces.
    pub fn get_vec(&self) -> Ev3Result<Vec<String>> {
        let value = self.get_str()?;
        let vec = value
            .split_whitespace()
            .map(|word| word.to_owned())
            .collect();
        Ok(vec)
    }

    /// Returns a C pointer to the wrapped file.
    pub fn get_raw_fd(&self) -> RawFd {
        self.file.lock().unwrap().as_raw_fd()
    }
}

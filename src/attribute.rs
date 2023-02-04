//! A wrapper to a attribute file commonly in the `/sys/class/` directory.
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::string::String;
use std::sync::{Arc, Mutex};

use crate::driver::DRIVER_PATH;
use crate::utils::OrErr;
use crate::{Ev3Error, Ev3Result};

/// A wrapper to a attribute file in the `/sys/class/` directory.
#[derive(Debug, Clone)]
pub struct Attribute {
    file_path: PathBuf,
    file: Arc<Mutex<File>>,
}

impl Attribute {
    /// Create a new `Attribute` instance for the given path.
    pub fn from_path(path: &Path) -> Ev3Result<Attribute> {
        let stat = fs::metadata(path)?;

        let mode = stat.permissions().mode();

        // Read permission for group (`ev3dev`)
        let readable = mode & 0o040 == 0o040;
        let writeable = mode & 0o020 == 0o020;

        let file = OpenOptions::new()
            .read(readable)
            .write(writeable)
            .open(path)?;

        Ok(Attribute {
            file_path: PathBuf::from(path),
            file: Arc::new(Mutex::new(file)),
        })
    }

    /// Create a new `Attribute` instance that wrap's
    /// the file `/sys/class/{class_name}/{name}{attribute_name}`.
    pub fn from_sys_class(
        class_name: &str,
        name: &str,
        attribute_name: &str,
    ) -> Ev3Result<Attribute> {
        let path = Path::new(DRIVER_PATH)
            .join(class_name)
            .join(name)
            .join(attribute_name);
        Attribute::from_path(path.as_ref())
    }

    /// Create a new `Attribute` instance by a discriminator attribute.
    /// This can be used to manually access driver files or advances features like raw encoder values.
    /// To find the correct file, this function iterates over all directories `$d` in `driver_path` and
    /// checks if the content of `driver_path/$d/discriminator_path` equals `discriminator_value`. When a
    /// match is found it returns an Attribute for file `driver_path/$d/attribute_path`.
    ///
    /// # Example
    /// ```no_run
    /// use ev3dev_lang_rust::Attribute;
    ///
    /// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    /// // Get value0 of first connected color sensor.
    /// let color_sensor_value = Attribute::from_path_with_discriminator(
    ///     "/sys/class/lego-sensor",
    ///     "value0",
    ///     "driver_name",
    ///     "lego-ev3-color"
    /// )?;
    /// println!("value0 of color sensor: {}", color_sensor_value.get::<i32>()?);
    ///
    /// // Get raw rotation count of motor in port `A`.
    /// // See https://github.com/ev3dev/ev3dev/wiki/Internals:-ev3dev-stretch for more information.
    /// let rotation_count = Attribute::from_path_with_discriminator(
    ///     "/sys/bus/iio/devices",
    ///     "in_count0_raw",
    ///     "name",
    ///     "ev3-tacho"
    /// )?;
    /// println!("Raw rotation count: {}", rotation_count.get::<i32>()?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_path_with_discriminator(
        driver_path: &str,
        attribute_path: &str,
        discriminator_path: &str,
        discriminator_value: &str,
    ) -> Ev3Result<Attribute> {
        let paths = fs::read_dir(driver_path)?;

        for path_result in paths {
            let path_buf = path_result?.path();
            let current_path = path_buf.to_str().or_err()?;

            let discriminator_attribute = Attribute::from_path(&PathBuf::from(format!(
                "{current_path}/{discriminator_path}"
            )))?;

            if discriminator_attribute.get::<String>()? == discriminator_value {
                return Attribute::from_path(&PathBuf::from(format!(
                    "{current_path}/{attribute_path}"
                )));
            }
        }

        Err(Ev3Error::InternalError {
            msg: format!(
                "Attribute `{attribute_path}` at driver path `{driver_path}` could not be found!"
            ),
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
            Err(err) => Err(Ev3Error::InternalError {
                msg: format!("{err}"),
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
    /// The file value is splitted at whitespace's.
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

    /// Returns the path to the wrapped file.
    pub fn get_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

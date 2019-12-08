use std::cell::RefCell;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::io::{AsRawFd, RawFd};
use std::rc::Rc;
use std::string::String;

use crate::{Ev3Error, Ev3Result};

pub const ROOT_PATH: &str = "/sys/class/";

#[derive(Clone)]
pub struct Attribute {
    file: Rc<RefCell<File>>,
}

impl Attribute {
    pub fn new(class_name: &str, name: &str, attribute_name: &str) -> Ev3Result<Attribute> {
        let file = Attribute::open_file(class_name, name, attribute_name)?;

        Ok(Attribute {
            file: Rc::new(RefCell::new(file)),
        })
    }

    fn open_file(class_name: &str, name: &str, attribute_name: &str) -> Ev3Result<File> {
        let mut filename = ROOT_PATH.to_owned();
        filename.push_str(class_name);
        filename.push_str("/");
        filename.push_str(name);
        filename.push_str("/");
        filename.push_str(attribute_name);

        let stat = fs::metadata(&filename)?;

        let mode = stat.permissions().mode();

        let readable = mode & 256 == 256;
        let writeable = mode & 128 == 128;

        Ok(OpenOptions::new()
            .read(readable)
            .write(writeable)
            .open(&filename)?)
    }

    fn get_str(&self) -> Ev3Result<String> {
        let mut value = String::new();
        let mut file = self.file.borrow_mut();
        file.seek(SeekFrom::Start(0))?;
        file.read_to_string(&mut value)?;
        Ok(value.trim_end().to_owned())
    }

    fn set_str(&self, value: &str) -> Ev3Result<()> {
        let mut file = self.file.borrow_mut();
        file.seek(SeekFrom::Start(0))?;
        file.write_all(value.as_bytes())?;
        Ok(())
    }

    pub fn get<T>(&self) -> Ev3Result<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: Error,
    {
        let value = self.get_str()?;
        match value.parse::<T>() {
            Ok(value) => Ok(value),
            Err(e) => Err(Ev3Error::InternalError {
                msg: e.description().to_owned(),
            }),
        }
    }

    pub fn set<T>(&self, value: T) -> Ev3Result<()>
    where
        T: std::string::ToString,
    {
        self.set_str(&value.to_string())
    }

    #[inline]
    pub fn set_str_slice(&self, value: &str) -> Ev3Result<()> {
        self.set_str(value)
    }

    pub fn get_vec(&self) -> Ev3Result<Vec<String>> {
        let value = self.get_str()?;
        let vec = value
            .split_whitespace()
            .map(|word| word.to_owned())
            .collect();
        Ok(vec)
    }

    pub fn get_raw_fd(&self) -> RawFd {
        self.file.borrow().as_raw_fd()
    }
}

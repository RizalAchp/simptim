#![allow(unused)]

use std::{error, fmt, result};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    SysTime(std::time::SystemTimeError),

    Static(&'static str),
    Unknown,
}
pub type Result<T = ()> = result::Result<T, Error>;

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IoError: {err}"),
            Error::SysTime(t) => write!(f, "SysTime: {t}"),
            Error::Static(s) => f.write_str(s),
            Error::Unknown => f.write_str("Unknown Error"),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

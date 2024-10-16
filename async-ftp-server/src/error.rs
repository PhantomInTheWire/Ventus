use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::result;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use futures::stream::ReuniteError;
use tokio_util::codec::Framed;
use crate::codec::FtpCodec;
use crate::ftp::Answer;

use self::Error::*;

#[derive(Debug)]
pub enum Error {
    FromUtf8(FromUtf8Error),
    Io(io::Error),
    Msg(String),
    Utf8(Utf8Error),
    Reunite(String),
}

impl Error {
    pub fn to_io_error(self) -> io::Error {
        match self {
            Io(error) => error,
            FromUtf8(_) | Msg(_) | Utf8(_) | Reunite(_) => io::ErrorKind::Other.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            FromUtf8(ref error) => error.fmt(formatter),
            Io(ref error) => error.fmt(formatter),
            Utf8(ref error) => error.fmt(formatter),
            Msg(ref msg) => write!(formatter, "{}", msg),
            Reunite(ref msg) => write!(formatter, "ReuniteError: {}", msg),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            FromUtf8(ref error) => Some(error),
            Io(ref error) => Some(error),
            Utf8(ref error) => Some(error),
            Msg(_) | Reunite(_) => None,
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Io(error)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(message: &'a str) -> Self {
        Msg(message.to_string())
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Utf8(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        FromUtf8(error)
    }
}

impl<T> From<ReuniteError<Framed<T, FtpCodec>, Answer>> for Error
where
    T: fmt::Debug,
{
    fn from(err: ReuniteError<Framed<T, FtpCodec>, Answer>) -> Self {
        Reunite(format!("{:?}", err))
    }
}
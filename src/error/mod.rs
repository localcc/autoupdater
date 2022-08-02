use std::{error, fmt::Display, io};

use reqwest::StatusCode;

#[derive(Debug)]
pub enum ErrorCode {
    NoRelease,
    Http(StatusCode),
    Reqwest(reqwest::Error),
    Io(io::Error),
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Reqwest(e) => write!(f, "{}", e),
            ErrorCode::NoRelease => write!(f, "Failed to find a release matching requirements"),
            ErrorCode::Io(e) => write!(f, "{}", e),
            ErrorCode::Http(code) => write!(f, "HTTP Response code: {}", code),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
}

impl Error {
    pub fn no_release() -> Self {
        Error {
            code: ErrorCode::NoRelease,
        }
    }

    pub fn http(code: StatusCode) -> Self {
        Error {
            code: ErrorCode::Http(code),
        }
    }
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error {
            code: ErrorCode::Reqwest(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            code: ErrorCode::Io(e),
        }
    }
}

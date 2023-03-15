use std::{error, fmt::Display, io};

#[derive(Debug)]
pub enum ErrorCode {
    NoRelease,
    Http(String),
    Ureq(Box<ureq::Error>),
    Io(io::Error),
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Ureq(e) => write!(f, "{e}"),
            ErrorCode::NoRelease => write!(f, "failed to find a release matching requirements"),
            ErrorCode::Io(e) => write!(f, "{e}"),
            ErrorCode::Http(e) => write!(f, "{e}"),
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

    pub fn http(code: &str) -> Self {
        Error {
            code: ErrorCode::Http(code.to_string()),
        }
    }
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Error {
            code: ErrorCode::Ureq(Box::new(e)),
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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to find a release matching requirements")]
    NoRelease,
    #[error("HTTP response code {0}")]
    Http(String),
    #[error("ureq: {0}")]
    Ureq(Box<ureq::Error>),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        Self::Ureq(Box::new(value))
    }
}

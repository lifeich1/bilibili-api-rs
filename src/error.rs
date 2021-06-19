use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    Remote(String),
}

impl Error {
    pub fn remote_err<T: ToString>(msg: T) -> Self {
        Self::Remote(msg.to_string())
    }

    pub fn is_network(&self) -> bool {
        matches!(self, Self::Network(_))
    }

    pub fn as_network(&self) -> Option<&reqwest::Error> {
        match self {
            Self::Network(e) => Some(&e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Network(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Network(e) => e.fmt(f),
            Self::Remote(s) => write!(f, "Remote: {}", &s),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Network(e) => e.source(),
            _ => None,
        }
    }
}

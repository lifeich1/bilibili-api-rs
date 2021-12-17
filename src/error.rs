use std::error::Error as StdError;
use std::fmt;

pub type ApiResult<T> = std::result::Result<T, crate::error::ApiError>;

#[derive(Debug)]
pub enum ApiError {
    Network(reqwest::Error),
    Remote(Option<i64>, Option<String>),
    Join(tokio::task::JoinError),
    General(String),
}

impl ApiError {
    pub fn remote_err(code: Option<i64>, msg: Option<&str>) -> Self {
        Self::Remote(code, msg.map(str::to_string))
    }

    pub fn general<T: ToString>(msg: T) -> Self {
        Self::General(msg.to_string())
    }

    pub fn is_network(&self) -> bool {
        matches!(self, Self::Network(_))
    }

    pub fn as_network(&self) -> Option<&reqwest::Error> {
        match self {
            Self::Network(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        Self::Network(error)
    }
}

impl From<tokio::task::JoinError> for ApiError {
    fn from(error: tokio::task::JoinError) -> Self {
        Self::Join(error)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Network(e) => e.fmt(f),
            Self::Join(e) => e.fmt(f),
            Self::Remote(c, s) => write!(f, "Remote api error code {:?}: {:?}", c, s),
            Self::General(s) => write!(f, "{}", s),
        }
    }
}

impl StdError for ApiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Network(e) => e.source(),
            Self::Join(e) => e.source(),
            _ => None,
        }
    }
}

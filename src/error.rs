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
        match self {
            Self::Network(_) => true,
            _ => false,
        }
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

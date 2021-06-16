#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    Remote(String),
}

impl Error {
    pub fn remote_err<T>(msg: T) -> Self
    where
        String: From<T>,
    {
        Self::Remote(String::from(msg))
    }

    pub fn is_network(&self) -> bool {
        match self {
            Self::Network(e) => true,
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

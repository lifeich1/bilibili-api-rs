#[derive(Debug)]
pub enum ErrorKind {
    Network,
    Remote(String),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    net: Option<reqwest::Error>,
}

impl Error {
    pub fn new() -> Self {
        Self {
            net: None,
            kind: ErrorKind::Network,
        }
    }

    pub fn remote_err(detail: &str) -> Self {
        Self {
            kind: ErrorKind::Remote(String::from(detail)),
            ..Self::new()
        }
    }

    pub fn is_net(&self) -> bool {
        self.net.is_some()
    }

    pub fn as_net(&self) -> Option<&reqwest::Error> {
        self.net.as_ref()
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self {
            net: Some(error),
            ..Self::new()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_static() {
        assert!(!NIL_ERROR.is_net());
    }
}

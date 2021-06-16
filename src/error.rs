use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Error {
    net: Option<reqwest::Error>,
}

lazy_static! {
    static ref NIL_ERROR: Error = Error { net: None };
}

impl Error {
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
            ..*NIL_ERROR
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

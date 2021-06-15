#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod net;

pub struct Error {
    net_err: Option<reqwest::Error>,
}

impl Error {
    pub fn is_net(&self) -> bool {
        self.net_err.is_some()
    }
}

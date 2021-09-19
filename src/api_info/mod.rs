use lazy_static::lazy_static;

pub trait GetFromPath {
    fn get_from_path(&self, path: &str) -> &serde_json::Value;
}

impl GetFromPath for serde_json::Value {
    fn get_from_path(&self, path: &str) -> &Self {
        path.split('/').fold(self, |j, n| &j[n])
    }
}

macro_rules! def_data {
    ($name:ident, $file:literal) => {
        lazy_static! {
            static ref $name: serde_json::Value = serde_json::from_str(include_str!($file))
                .unwrap_or_else(|e| panic!("Parse api json error(s): {}", e));
        }

        pub fn get(path: &str) -> &serde_json::Value {
            $name.get_from_path(path)
        }
    };
    ($file:literal) => {
        def_data! { DATA, $file }
    };
}

pub mod user {
    use super::*;
    def_data! { "user.json" }
}

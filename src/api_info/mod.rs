pub mod user;

pub trait GetFromPath {
    fn get_from_path(&self, path: &str) -> &serde_json::Value;
}

impl GetFromPath for serde_json::Value {
    fn get_from_path(&self, path: &str) -> &Self {
        path.split('/').fold(self, |j, n| &j[n])
    }
}

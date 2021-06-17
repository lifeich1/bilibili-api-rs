pub mod user;

pub trait GetFromPath {
    fn get_from_path(&self, path: &str) -> &serde_json::Value;
}

impl GetFromPath for serde_json::Value {
    fn get_from_path(&self, path: &str) -> &serde_json::Value {
        let v = path.split('/');
        let j = self;
        for n in v {
            j = &j[n];
        }
        j
    }
}

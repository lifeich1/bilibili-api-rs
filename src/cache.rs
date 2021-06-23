use std::collections::HashMap;
use std::sync::Mutex;

pub trait Cacher {
    fn cache_store(&self, key: &str, val: &str);
    fn cache_get(&self, key: &str) -> Option<String>;
}

#[derive(Default)]
pub struct SimpleMemCacher {
    map: Mutex<HashMap<String, String>>,
}

impl Cacher for SimpleMemCacher {
    fn cache_store(&self, key: &str, val: &str) {
        self.map.lock().expect("SimpleMemCacher poisoned").insert(key.to_string(), val.to_string());
    }

    fn cache_get(&self, key: &str) -> Option<String> {
        self.map.lock().expect("SimpleMemCacher poisoned").get(key).cloned()
    }
}

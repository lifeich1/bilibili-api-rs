use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Cache interface that crate used.
///
/// Note: must implement it with multi-thread safety.
pub trait Cacher {
    fn cache_store(&self, _key: &str, _val: &str) {}

    fn cache_get(&self, _key: &str) -> Option<String> {
        None
    }
}

#[derive(Default)]
pub struct NoCacher {}

impl Cacher for NoCacher {
}


/// A simple in memory cacher which remember until poweroff.
#[derive(Default)]
pub struct SimpleMemCacher {
    pub map: Mutex<HashMap<String, String>>,
}

impl Cacher for SimpleMemCacher {
    fn cache_store(&self, key: &str, val: &str) {
        self.map
            .lock()
            .expect("SimpleMemCacher poisoned")
            .insert(key.to_string(), val.to_string());
    }

    fn cache_get(&self, key: &str) -> Option<String> {
        self.map
            .lock()
            .expect("SimpleMemCacher poisoned")
            .get(key)
            .cloned()
    }
}

/// A simple in memory cacher which only remember each item in a short duration.
pub struct SimpleFishMemCacher {
    pub map: Mutex<HashMap<String, (Instant, String)>>,
    pub forgot_duration: Duration,
}

impl SimpleFishMemCacher {
    pub fn new(forgot_duration: Duration) -> Self {
        Self {
            forgot_duration,
            ..Default::default()
        }
    }
}

impl Default for SimpleFishMemCacher {
    fn default() -> Self {
        Self {
            map: Default::default(),
            forgot_duration: Duration::from_secs(120),
        }
    }
}

impl Cacher for SimpleFishMemCacher {
    fn cache_store(&self, key: &str, val: &str) {
        self.map
            .lock()
            .expect("SimpleFishMemCacher poisoned")
            .insert(key.to_string(), (Instant::now(), val.to_string()));
    }

    fn cache_get(&self, key: &str) -> Option<String> {
        self.map
            .lock()
            .expect("SimpleFishMemCacher poisoned")
            .get(key)
            .filter(|(i, _)| i.elapsed() < self.forgot_duration)
            .map(|(_, v)| v)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_mem_cacher() {
        let c = SimpleMemCacher::default();
        c.cache_store("abc", "fed");
        assert_eq!(c.cache_get("abc"), Some(String::from("fed")));
    }

    #[test]
    fn test_simple_fish_mem_cacher() {
        let d = Duration::from_millis(100);
        let c = SimpleFishMemCacher::new(d);
        c.cache_store("abc", "fed");
        assert_eq!(c.cache_get("abc"), Some(String::from("fed")));

        std::thread::sleep(d);
        assert_eq!(c.cache_get("abc"), None);
    }
}

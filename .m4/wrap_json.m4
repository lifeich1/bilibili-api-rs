use crate::api_info::GetFromPath;
use lazy_static::lazy_static;
use serde_json::json;

lazy_static! {
    // Copy from https://github.com/Passkou/bilibili-api
    static ref DATA: serde_json::Value = json!(m4_dnl
m4_divert(1)m4_dnl
);
}

pub fn get_str(path: &str) -> &str {
    DATA.get_from_path(path).as_str().expect(&format!("api_info: path invalid: {}", path))
}

pub fn get(path: &str) -> &serde_json::Value {
    DATA.get_from_path(path)
}
m4_divert(0)m4_dnl

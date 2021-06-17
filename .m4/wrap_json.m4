lazy_static! {
    // Copy from https://github.com/Passkou/bilibili-api
    static ref DATA: serde_json::Value = json!(m4_dnl
m4_divert(1)m4_dnl
);
}

pub fn get(path: &str) -> &str {
    DATA.get_from_path(path).as_str().expect(&format!("api_info: path invalid: {}", path))
}
m4_divert(0)m4_dnl

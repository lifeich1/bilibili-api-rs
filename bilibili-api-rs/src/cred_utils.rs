use anyhow::Result;
use serde_json::{json, Value};

pub fn gen_buvid_fp(payload: &Value) -> Result<String> {
    let payload = payload.to_string();
    let m = murmur3::murmur3_x64_128(&mut payload.as_bytes(), 31)?;
    let hd = m & u128::from(u64::MAX);
    let hd = format!("{hd:x}");
    let tl = m >> 64;
    let tl = format!("{tl:x}");
    Ok(format!("{}{}", hd.split_at(2).1, tl.split_at(2).1))
}

pub fn gen_payload(uuid: &str) -> Value {
    let mut data: Value =
        serde_json::from_str(include_str!("cred_data.json")).expect("cred_data.json must be valid");
    data["df35"] = uuid.into();
    data["54ef"] = include_str!("cred_data_54ef.txt").into();
    data["5062"] = (i64::from(chrono::Local::now().timestamp_subsec_nanos()) / 1_000_000
        + chrono::Local::now().timestamp() * 1000)
        .into();
    json!({"payload": data.to_string()})
}

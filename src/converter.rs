use crate::LASER_JSON;
use serde_json::Value;

pub fn get_sha(ver: String) -> String {
    let data: Value = serde_json::from_str(LASER_JSON).unwrap();
    let mut latest = "";

    for version in data.as_object().unwrap().keys() {
        let major = version.split('.').next().unwrap();
        if major == ver {
            latest = version;
            continue;
        }
        if ver == *version {
            latest = version;
        }
    }
    if latest == "" {
        println!("[ERROR] Invalid version: {}", &ver);
        return "Wrong".to_owned();
    }
    let sha: String = data[latest].as_str().unwrap().to_string();
    return sha;
}
use crate::{LASER_JSON, SQUAD_JSON};
use serde_json::Value;

pub fn get_sha(ver: String, game: String) -> String {
    let mut data: Value = serde_json::from_str(LASER_JSON).unwrap();
    if (game == "laser") {
        data = serde_json::from_str(LASER_JSON).unwrap();
    } else if (game == "squad") {
        data = serde_json::from_str(SQUAD_JSON).unwrap();
    } else {
        panic!("Game not in the list");
    }
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
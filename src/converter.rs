use crate::{logger, LASER_JSON, SQUAD_JSON};
use serde_json::Value;

pub fn get_sha(ver: String, game: &String) -> String {
    let data: Value = match game.as_str() {
        "laser" => serde_json::from_str(LASER_JSON).unwrap(),
        "squad" => serde_json::from_str(SQUAD_JSON).unwrap(),
        _ => panic!("Game not in the list"),
    };

    let mut latest = "";
    for version in data.as_object().unwrap().keys() {
        if ver == *version || version.starts_with(&format!("{ver}.")) {
            latest = version;
        }
    }
    if latest == "" {
        logger::error(&format!("Invalid version: {}", &ver));
        return "Wrong".to_owned();
    }
    return data[latest].as_str().unwrap().to_string(); // sha
}
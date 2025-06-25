use crate::{converter::get_sha, downloader::download_file, logger, GAME_TYPE};
use reqwest::blocking::get;
use serde_json::Value;
use std::time::Instant;

pub fn download_asset(ver: String, asset: String, game: String) {
    let start = Instant::now();
    
    let game_type: Value = serde_json::from_str(GAME_TYPE).unwrap();

    let sha: String = get_sha(ver, &game);
    if sha == "Wrong" {
        return;
    }
    logger::info(&format!("SHA: {}", sha));

    let mut scfile: bool = false;
    if asset.ends_with(".sc") {
        scfile = true;
    }
    let download_url: String = format!("https://game-assets.{}game.com/{}/{}", game_type[&game].as_str().unwrap().to_string(), sha, asset);
    let fingerprint_url: String = format!("https://game-assets.{}game.com/{}/{}", game_type[&game].as_str().unwrap().to_string(), sha, "fingerprint.json");
    let finger_response = get(fingerprint_url).unwrap();
    let finger_response_text = finger_response.text().unwrap();
    
    let finger_text: Value = match serde_json::from_str(&finger_response_text) {
        Ok(v) => v,
        Err(e) => {
            let _ = e;
            logger::error("Supercell Asset Server for this version is down!");
            return;
            
        }
    };
    if let Some(files) = finger_text["files"].as_array() {
        for file_info in files {
            if let Some(file_name) = file_info.get("file").and_then(|v| v.as_str()) {
                if file_name == asset {
                    if scfile {
                        logger::info(&format!("Asset {} is sc file. Trying to download texture files", asset));
                        handle_scfile_download(&asset, files, sha, &game);
                    }
                    let _ = download_file(download_url, file_name.to_string());
                    logger::success(&format!("Done in {:?}", start.elapsed()));
                    return;
                }
            }
        }
    }
    logger::error(&format!("Asset \"{}\" not found in fingerprint.json", &asset));
}

fn handle_scfile_download(asset: &str, files: &Vec<Value>, sha: String, game: &String) {
    let base = asset.strip_suffix(".sc").unwrap();
    let game_type: Value = serde_json::from_str(GAME_TYPE).unwrap();
    let variants = [
        format!("{}_highres_tex.sc", base),
        format!("{}_lowres_tex.sc", base),
        format!("{}_tex.sc", base),
    ];
    let mut downloaded = false;
    for variant in &variants {
        if files.iter().any(|f| f.get("file").and_then(|v| v.as_str()) == Some(variant)) {
            let url = format!("https://game-assets.{}game.com/{}/{}", game_type[&game].as_str().unwrap().to_string(),sha, variant);
            if download_file(url, variant.clone()).is_ok() {
                downloaded = true;
            }
        }
    } 
    if !downloaded {
        logger::info(&format!("[INFO] {} has no texture files. The textures are probably in the asset", asset));
    }
}
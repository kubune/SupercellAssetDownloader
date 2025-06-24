use crate::{downloader::DownloadFile, converter::get_sha};
use reqwest::blocking::get;
use serde_json::Value;
use std::time::Instant;

pub fn DownloadAsset(ver: String, asset: String) {
    let start = Instant::now();
    
    let sha: String = get_sha(ver, "squad".to_owned());
    if sha == "Wrong" {
        return;
    }
    println!("[INFO] SHA: {}", sha);

    let mut SCFile: bool = false;
    if asset.ends_with(".sc") {
        SCFile = true;
    }
    let DownloadURL: String = format!("https://game-assets.squadbustersgame.com/{}/{}", sha, asset);
    let FingerprintURL: String = format!("https://game-assets.squadbustersgame.com/{}/{}", sha, "fingerprint.json");
    let finger_response = get(FingerprintURL).unwrap();
    let finger_response_text = finger_response.text().unwrap();
    
    let finger_text: Value = match serde_json::from_str(&finger_response_text) {
        Ok(v) => v,
        Err(e) => {
            let _ = e;
            eprintln!("[ERROR] Supercell Asset Server for this version is down!");
            return;
        }
    };
    if let Some(files) = finger_text["files"].as_array() {
        for fileInfo in files {
            if let Some(file_name) = fileInfo.get("file").and_then(|v| v.as_str()) {
                if file_name == asset {
                    if SCFile {
                        println!("[INFO] Asset {} is sc file. Trying to download texture files", asset);
                        HandleSCFileDownload(asset, files, sha);
                    }
                    DownloadFile(DownloadURL, file_name.to_string());
                    println!("[SUCCESS] Done in {:?}", start.elapsed());
                    return;
                }
            }
        }
    }
    println!("[ERROR] Asset \"{}\" not found in fingerprint.json", &asset);
}

fn HandleSCFileDownload(asset: String, files: &Vec<Value>, sha: String) {
    let base = asset.strip_suffix(".sc").unwrap();
    let variants = [
        format!("{}_highres_tex.sc", base),
        format!("{}_lowres_tex.sc", base),
        format!("{}_tex.sc", base),
    ];
    let mut downloaded = false;
    for variant in &variants {
        if files.iter().any(|f| f.get("file").and_then(|v| v.as_str()) == Some(variant)) {
            let url = format!("https://game-assets.squadbustersgame.com/{}/{}", sha, variant);
            if DownloadFile(url, variant.clone()).is_ok() {
                downloaded = true;
            }
        }
    }
    if !downloaded {
        println!("[INFO] {} has no texture files. The textures are probably in the asset", asset);
    }
}
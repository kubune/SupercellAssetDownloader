use std::{fs::File, io::Write};
use reqwest::blocking::get;

use crate::logger;

pub fn download_file(url: String, asset: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(&url)?;
    let downloaded_file = response.bytes()?;
    let mut file  = File::create(&asset.split("/").last().unwrap())?;
    file.write_all(&downloaded_file)?;

    logger::success(&format!("Downloaded {}!", asset));
    return Ok(());
}
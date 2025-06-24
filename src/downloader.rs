use std::{fs::File, io::Write};
use reqwest::blocking::get;

pub fn DownloadFile(url: String, asset: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(&url)?;
    let downloadedFile = response.bytes()?;

    let mut file  = File::create(&asset.split("/").last().unwrap())?;
    file.write_all(&downloadedFile)?;

    println!("[SUCCESS] Downloaded {}!", asset);
    return Ok(());
}
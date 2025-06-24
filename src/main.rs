use clap::Parser;
mod downloader;
mod converter;
mod laser;
mod squad;

pub const LASER_JSON: &str = include_str!("json/laser.json");
pub const SQUAD_JSON: &str = include_str!("json/squad.json");

#[derive(Parser, Debug)]
#[command(name = "SupercellAssetsDownloader")]
#[command(about = "Tool to download assets from Supercell games made by kubune", long_about = None)]
struct Args {
    #[arg(short, long)]
    game: String,

    #[arg(short, long)]
    version: String,

    #[arg(short, long)]
    asset: String,
}

fn main() {
    let args = Args::parse();
    println!("Downloading...\nSelected Game: {}\nSelected Asset: {}", args.game, args.asset);
    if args.game == "laser" {
        laser::DownloadAsset(args.version, args.asset);
    } else if args.game == "squad" {
        squad::DownloadAsset(args.version, args.asset);
    }
}

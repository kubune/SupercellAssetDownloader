use clap::Parser;
mod logger;
mod downloader;
mod converter;
mod game;

pub const LASER_JSON: &str = include_str!("json/laser.json");
pub const SQUAD_JSON: &str = include_str!("json/squad.json");

pub const GAME_TYPE: &str = include_str!("json/game_types.json");

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
    logger::info(&format!("Downloading...\nSelected Game: {}\nSelected Asset: {}", args.game, args.asset));
    game::download_asset(args.version, args.asset, args.game);
}

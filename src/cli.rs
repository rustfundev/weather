use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Latitude of the geographic coordinate
    #[arg(short, long)]
    pub latitude: f32,

    /// Longitude of the geographic coordinate
    #[arg(short = 'L', long)]
    pub longitude: f32,
}

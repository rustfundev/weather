mod cli;
mod coordinate;
mod request;
mod weather;

use clap::Parser;

use cli::Args;
use coordinate::Coordinate;
use request::Request;
use weather::get_weather;

struct WeatherRequest;

impl Request for WeatherRequest {}

fn main() {
    let args = Args::parse();
    let coord: Coordinate = Coordinate {
        latitude: args.latitude,
        longitude: args.longitude,
    };
    println!("{:?}", get_weather(&WeatherRequest, &coord));
}

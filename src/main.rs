mod cli;
mod coordinate;
mod request;
mod weather;

use clap::Parser;

use cli::Args;
use coordinate::Coordinate;
use request::Request;
use weather::get_weather;
use weather::SkyCover;

struct WeatherRequest;

impl Request for WeatherRequest {}

fn main() {
    let args = Args::parse();
    let coord: Coordinate = Coordinate {
        latitude: args.latitude,
        longitude: args.longitude,
    };
    let weather = get_weather(&WeatherRequest, &coord).expect("Failed");
    let sky = SkyCover::new(weather.current.cloud_cover, weather.current.is_day);
    println!(
        "Currently, the temperature is {} C and the sky is {}",
        weather.current.temperature_2m, sky
    );
}

mod cli;
mod coordinate;
mod request;
mod weather;

use clap::Parser;

use cli::Args;
use coordinate::Coordinate;
use request::Request;
use weather::get_weather;

struct MyRequest;

impl Request for MyRequest {}

fn main() {
    let args = Args::parse();
    let my_req = MyRequest;
    let coord: Coordinate = Coordinate {
        latitude: args.latitude,
        longitude: args.longitude,
    };
    println!("{:?}", get_weather(&my_req, &coord));
}

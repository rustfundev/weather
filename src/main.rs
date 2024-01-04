mod coordinate;
mod request;
mod weather;

use coordinate::Coordinate;
use request::Request;
use weather::get_weather;

struct MyRequest;

impl Request for MyRequest {}

fn main() {
    let my_req = MyRequest;
    let coord: Coordinate = Coordinate {
        latitude: 10.0,
        longitude: 10.0,
    };
    println!("{:?}", get_weather(&my_req, &coord));
}

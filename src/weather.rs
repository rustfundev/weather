use crate::coordinate::Coordinate;
use crate::request::Request;

use serde::{Deserialize, Serialize};

use std::fmt::{self};

const WEATHER_SERVICE_API: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    interval: u32,
    pub temperature_2m: f64,
    time: String,
    pub cloud_cover: u8,
    pub is_day: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub current: CurrentWeather,
}

pub enum SkyCover {
    Cloudy,
    MostlyCloudy,
    PartlyCloudy,
    PartlySunny,
    MostlySunny,
    MostlyClear,
    Clear,
    Undefined,
}

impl fmt::Display for SkyCover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SkyCover::Cloudy => write!(f, "cloudy"),
            SkyCover::MostlyCloudy => write!(f, "mostly cloudy"),
            SkyCover::PartlyCloudy => write!(f, "partly cloudy"),
            SkyCover::PartlySunny => write!(f, "partly sunny"),
            SkyCover::MostlySunny => write!(f, "mostly sunny"),
            SkyCover::MostlyClear => write!(f, "mostly clear"),
            SkyCover::Clear => write!(f, "clear"),
            _ => write!(f, "undefined sky cover"),
        }
    }
}

impl SkyCover {
    pub fn new(cloud_cover: u8, is_day: u8) -> Self {
        match cloud_cover {
            0_u8..=10u8 => SkyCover::Clear,
            11u8..=30u8 => {
                if is_day == 1 {
                    SkyCover::MostlySunny
                } else {
                    SkyCover::MostlyClear
                }
            }
            31u8..=70u8 => {
                if is_day == 1 {
                    SkyCover::PartlySunny
                } else {
                    SkyCover::PartlyCloudy
                }
            }
            71_u8..=90_u8 => SkyCover::MostlyCloudy,
            91u8..=100u8 => SkyCover::Cloudy,
            _ => SkyCover::Undefined,
        }
    }
}

pub fn get_weather(
    request: &impl Request,
    coord: &Coordinate,
) -> Result<Weather, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{0}?latitude={1}&longitude={2}&current=temperature_2m,cloud_cover,is_day",
        WEATHER_SERVICE_API, coord.latitude, coord.longitude
    );
    let result = request.get(url).expect("Error requesting");
    let w: Weather = serde_json::from_str(&result)?;
    Ok(w)
}

#[cfg(test)]
mod t {
    use super::*;

    use crate::request::MockRequest;
    use mockall::predicate::eq;

    #[test]
    fn test_get_weather() {
        let mut mock: MockRequest = MockRequest::new();
        let url = String::from("https://api.open-meteo.com/v1/forecast?latitude=10&longitude=10&current=temperature_2m,cloud_cover,is_day");
        mock.expect_get().with(eq(url)).times(1).returning(|_| {
            Ok(String::from(
                "{\"current\": {\"interval\": 10, \"temperature_2m\": 10.0, \"time\": \"10\", \"cloud_cover\": 90, \"is_day\": 1}}",
            ))
        });

        let coord = Coordinate {
            latitude: 10.0,
            longitude: 10.0,
        };

        let result = get_weather(&mock, &coord).expect("Error parsing");
        assert_eq!(result.current.temperature_2m, 10.0);
    }
}

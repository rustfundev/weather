use crate::coordinate::Coordinate;
use crate::request::Request;

use serde::{Deserialize, Serialize};

use std::fmt::{self};

const WEATHER_SERVICE_API: &str = "https://api.open-meteo.com/v1/forecast";

const FIELDS_TO_DISPLAY: &str = "temperature_2m,cloud_cover,is_day";

#[derive(Debug)]
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

#[derive(Serialize, Deserialize)]
struct DataCurrentWeather {
    interval: u32,
    pub temperature_2m: f32,
    time: String,
    pub cloud_cover: u8,
    pub is_day: u8,
}

#[derive(Serialize, Deserialize)]
struct DataWeather {
    current: DataCurrentWeather,
}

#[derive(Debug)]
pub struct Weather {
    pub temperature: f32,
    pub sky_cover: SkyCover,
}

fn url(latitude: f32, longitude: f32) -> String {
    let url: String = format!(
        "{0}?latitude={1}&longitude={2}&current={3}",
        WEATHER_SERVICE_API, latitude, longitude, FIELDS_TO_DISPLAY
    );

    url
}

pub fn get_weather(
    request: &impl Request,
    coord: &Coordinate,
) -> Result<Weather, Box<dyn std::error::Error>> {
    let result = request
        .get(url(coord.latitude, coord.longitude))
        .expect("Error requesting");
    let w: DataWeather = serde_json::from_str(&result)?;
    Ok(Weather {
        temperature: w.current.temperature_2m,
        sky_cover: SkyCover::new(w.current.cloud_cover, w.current.is_day),
    })
}

#[cfg(test)]
mod t {
    use super::*;

    use crate::request::MockRequest;
    use mockall::predicate::eq;

    #[test]
    fn test_get_weather() {
        let mut mock: MockRequest = MockRequest::new();
        let weather = DataWeather {
            current: DataCurrentWeather {
                interval: 10,
                temperature_2m: 10.0,
                time: String::from("10"),
                cloud_cover: 90,
                is_day: 1,
            },
        };
        let result: String = serde_json::to_string(&weather).unwrap();
        let url = url(10.0, 10.0);
        mock.expect_get()
            .with(eq(url))
            .times(1)
            .returning(move |_| Ok(result.clone()));

        let coord = Coordinate {
            latitude: 10.0,
            longitude: 10.0,
        };

        let result = get_weather(&mock, &coord).expect("Error parsing");
        assert_eq!(result.temperature, 10.0);
    }
}

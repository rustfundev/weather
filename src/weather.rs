use crate::coordinate::Coordinate;
use crate::request::Request;

use serde::{Deserialize, Serialize};

const WEATHER_SERVICE_API: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    interval: u32,
    temperature_2m: f64,
    time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    current: CurrentWeather,
}

pub fn get_weather(
    request: &impl Request,
    coord: &Coordinate,
) -> Result<Weather, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{0}?latitude={1}&longitude={2}&current=temperature_2m",
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
        let url = String::from("https://api.open-meteo.com/v1/forecast?latitude=10&longitude=10&current=temperature_2m");
        mock.expect_get().with(eq(url)).times(1).returning(|_| {
            Ok(String::from(
                "{\"current\": {\"interval\": 10, \"temperature_2m\": 10.0, \"time\": \"10\"}}",
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

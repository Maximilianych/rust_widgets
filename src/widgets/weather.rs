use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json;

pub mod prelude {
    pub use super::weather_request;
    pub use super::Weather;
    pub use super::Current;
    pub use super::CurrentUnits;
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Weather {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub current_units: CurrentUnits,
    pub current: Current,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CurrentUnits {
    pub time: String,
    pub interval: String,
    pub temperature_2m: String,
    pub relative_humidity_2m: String,
    pub apparent_temperature: String,
    pub is_day: String,
    pub precipitation: String,
    pub rain: String,
    pub showers: String,
    pub snowfall: String,
    pub weather_code: String,
    pub cloud_cover: String,
    pub surface_pressure: String,
    pub wind_speed_10m: String,
    pub wind_direction_10m: String,
    pub wind_gusts_10m: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Current {
    pub time: String,
    pub interval: u64,
    pub temperature_2m: f64,
    pub relative_humidity_2m: u64,
    pub apparent_temperature: f64,
    pub is_day: u64,
    pub precipitation: f64,
    pub rain: f64,
    pub showers: f64,
    pub snowfall: f64,
    pub weather_code: u64,
    pub cloud_cover: u64,
    pub surface_pressure: f64,
    pub wind_speed_10m: f64,
    pub wind_direction_10m: u64,
    pub wind_gusts_10m: f64,
}

pub fn weather_request(client: &mut Client) -> Weather {
    let weather = {
        let res = client.get("https://api.open-meteo.com/v1/forecast?latitude=59.9386&longitude=30.3141&current=temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,rain,showers,snowfall,weather_code,cloud_cover,surface_pressure,wind_speed_10m,wind_direction_10m,wind_gusts_10m&timezone=auto")
        .send()
        .unwrap()
        .text()
        .unwrap();
        serde_json::from_str(&res).unwrap()
    };

    weather
}
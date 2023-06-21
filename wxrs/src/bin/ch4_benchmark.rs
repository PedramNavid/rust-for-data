// ANCHOR: all

// ANCHOR: structs
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AirPollution {
    pub coord: Coord,
    pub list: Vec<List>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub main: Main,
    pub components: Components,
    pub dt: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Main {
    pub aqi: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Components {
    pub co: f32,
    pub no: f32,
    pub no2: f32,
    pub o3: f32,
    pub so2: f32,
    pub pm2_5: f32,
    pub pm10: f32,
    pub nh3: f32,
}
// ANCHOR_END: structs

// ANCHOR: forecast
// ANCHOR: get_json
pub fn get_air_pollution(lat: f32, lon: f32) -> AirPollution {
    let api_key = std::env::var("OWM_APPID").expect(
        "Environment Variable OWM_APPID not set. Please set it to your
    OpenWeatherMap API key. https://home.openweathermap.org/api_keys",
    );

    let url = format!(
        "http://api.openweathermap.org/data/2.5/air_pollution/forecast?lat={}&lon={}&appid={}",
        lat, lon, api_key
    );

    reqwest::blocking::get(url)
        .expect("request failed")
        .json()
        .expect("json failed")
    // ANCHOR_END: get_json
}

// ANCHOR: parse_air
pub fn parse_air_pollution(body: AirPollution) -> Vec<(Main, Components, usize)> {
    body.list
        .iter()
        .map(|x| (x.main, x.components, x.dt))
        .collect()
}
// ANCHOR_END: parse_air
// ANCHOR_END: forecast

pub fn print_air_pollution(main: Main, components: Components, dt: usize) {
    println!("---");
    println!("Weather info for date: {}", dt);
    println!("AQI: {}", main.aqi);
    println!("CO: {}", components.co);
    println!("NO: {}", components.no);
    println!("NO2: {}", components.no2);
    println!("O3: {}", components.o3);
    println!("SO2: {}", components.so2);
    println!("PM2.5: {}", components.pm2_5);
    println!("PM10: {}", components.pm10);
    println!("NH3: {}", components.nh3);
}

pub fn main() {
    let usage = format!("Usage: {} [lat] [lon]", std::env::args().next().unwrap());

    let lat = std::env::args()
        .nth(1)
        .expect(&usage)
        .parse::<f32>()
        .expect(&usage);

    let lon = std::env::args()
        .nth(2)
        .expect(&usage)
        .parse::<f32>()
        .expect(&usage);

    let body = get_air_pollution(lat, lon);
    let results = parse_air_pollution(body);

    for (main, components, dt) in results {
        print_air_pollution(main, components, dt);
    }
}
// ANCHOR_END: all

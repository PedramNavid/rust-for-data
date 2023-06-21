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

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    pub aqi: u8,
}

#[derive(Debug, Serialize, Deserialize)]
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

// ANCHOR: get_json
pub fn get_air_pollution(lat: f32, lon: f32) -> AirPollution {
    let api_key = std::env::var("OWM_APPID").expect(
        "Environment Variable OWM_APPID not set. Please set it to your
    OpenWeatherMap API key. https://home.openweathermap.org/api_keys",
    );

    let url = format!(
        "http://api.openweathermap.org/data/2.5/air_pollution?lat={}&lon={}&appid={}",
        lat, lon, api_key
    );

    reqwest::blocking::get(url)
        .expect("request failed")
        .json()
        .expect("json failed")
    // ANCHOR_END: get_json
}

// ANCHOR: parse_air
pub fn parse_air_pollution(body: &AirPollution) -> (&Main, &Components) {
    let main = &body.list[0].main;
    let components = &body.list[0].components;
    (main, components)
}
// ANCHOR_END: parse_air

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
    let (main, components) = parse_air_pollution(&body);

    println!("Air Quality Index: {}", main.aqi);

    println!("Carbon Monoxide: {} μg/m³", components.co);
    println!("Nitrogen Monoxide: {} μg/m³", components.no);
    println!("Nitrogen Dioxide: {} μg/m³", components.no2);
    println!("Ozone: {} μg/m³", components.o3);
    println!("Sulfur Dioxide: {} μg/m³", components.so2);
    println!("Particulate Matter < 2.5 μm: {} μg/m³", components.pm2_5);
    println!("Particulate Matter < 10 μm: {} μg/m³", components.pm10);
    println!("Ammonia: {} μg/m³", components.nh3);
}
// ANCHOR_END: all

// ANCHOR: all

use std::{
    fs::File,
    io::{BufWriter, Read, Write},
};

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
pub fn get_air_pollution() -> AirPollution {
    let mut script_path = std::env::current_exe()
        .expect("Failed to get current execution path")
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf();

    script_path.push("../../../lib/big_payload.json");

    let mut file = File::open(&script_path).expect("Failed to open file");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    serde_json::from_str(&content).unwrap()
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

// Takes a writer rather than calling println! directly. Rust's stdout is line
// buffered, so println! in a hot loop costs one write syscall per line; wrapping
// it in a BufWriter is the idiomatic fix. See the note in chapter 4.
pub fn print_air_pollution<W: Write>(w: &mut W, main: Main, components: Components, dt: usize) {
    writeln!(w, "---").unwrap();
    writeln!(w, "Weather info for date: {}", dt).unwrap();
    writeln!(w, "AQI: {}", main.aqi).unwrap();
    writeln!(w, "CO: {}", components.co).unwrap();
    writeln!(w, "NO: {}", components.no).unwrap();
    writeln!(w, "NO2: {}", components.no2).unwrap();
    writeln!(w, "O3: {}", components.o3).unwrap();
    writeln!(w, "SO2: {}", components.so2).unwrap();
    writeln!(w, "PM2.5: {}", components.pm2_5).unwrap();
    writeln!(w, "PM10: {}", components.pm10).unwrap();
    writeln!(w, "NH3: {}", components.nh3).unwrap();
}

pub fn main() {
    let body = get_air_pollution();
    let results = parse_air_pollution(body);

    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for (main, components, dt) in results {
        print_air_pollution(&mut out, main, components, dt);
    }
}
// ANCHOR_END: all

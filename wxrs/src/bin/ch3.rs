// ANCHOR: all
use std::time::Duration;

// ANCHOR: fetch
pub fn get_air_pollution(lat: f32, lon: f32, api_key: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/air_pollution?lat={}&lon={}&appid={}",
        lat, lon, api_key
    );

    let body = client.get(url).send()?.error_for_status()?.text()?;

    Ok(body)
}
// ANCHOR_END: fetch

pub fn main() {
    let usage = format!("Usage: {} [lat] [lon]", std::env::args().next().unwrap());

    let api_key = std::env::var("OWM_APPID").expect(
        "Environment Variable OWM_APPID not set. Please set it to your
    OpenWeatherMap API key. https://home.openweathermap.org/api_keys",
    );

    // ANCHOR: parse
    let lat = std::env::args()
        .nth(1)
        .expect(&usage)
        .parse::<f32>()
        .expect(&usage);
    // ANCHOR_END: parse

    let lon = std::env::args()
        .nth(2)
        .expect(&usage)
        .parse::<f32>()
        .expect(&usage);

    // ANCHOR: call
    match get_air_pollution(lat, lon, &api_key) {
        Ok(body) => println!("{}", body),
        Err(err) => {
            eprintln!("Request failed: {}", err);
            std::process::exit(1);
        }
    }
    // ANCHOR_END: call
}
// ANCHOR_END: all

pub fn get_air_pollution(lat: f32, lon: f32) -> String {
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
        .text()
        .expect("body failed")
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
    println!("{}", body);
}

fn main() {
    let lat = 37.9871;
    let lon = -122.5889;
    let appid = std::env::var("OWM_APPID").expect(
        "Environment Variable OWM_APPID not set. Please set it to your
        OpenWeatherMap API key. https://home.openweathermap.org/api_keys",
    );
    let url = format!(
        "http://api.openweathermap.org/data/2.5/air_pollution?lat={}&lon={}&appid={}",
        lat, lon, appid
    );
    let body = reqwest::blocking::get(url)
        .expect("request failed")
        .text()
        .expect("body failed");

    println!("{}", body);
}

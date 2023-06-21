## ANCHOR: all
import os
import sys
import requests

API_KEY = os.getenv("OWM_APPID")


def get_air_pollution(lat, lon):
    url = f"http://api.openweathermap.org/data/2.5/air_pollution?lat={lat}&lon={lon}&appid={API_KEY}"
    body = requests.get(url).json()
    return body

## ANCHOR: parse_air
def parse_air_pollution(body):
    aqi = body["list"][0]["main"]["aqi"]
    components = body["list"][0]["components"]
    return (aqi, components)
## ANCHOR_END: parse_air

if __name__ == "__main__":
    usage = f"Usage: python {__file__} <lat> <lon>"

    if not API_KEY:
        print("Please set OWM_APPID environment variable")
        sys.exit(1)

    if len(sys.argv) != 3:
        print(usage)
        sys.exit(1)

    lat = sys.argv[1]
    lon = sys.argv[2]
    body = get_air_pollution(lat, lon)
    aqi, components = parse_air_pollution(body)

    print(f"Air Quality Index: {aqi}")
    print("Components:")
    for k, v in components.items():
        print(f"  {k}: {v}")

## ANCHOR_END: all

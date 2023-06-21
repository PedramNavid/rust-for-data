## ANCHOR: all
import os
import sys
import requests

API_KEY = os.getenv("OWM_APPID")


## ANCHOR: forecast
def get_air_pollution(lat, lon):
    url = f"http://api.openweathermap.org/data/2.5/air_pollution/forecast?lat={lat}&lon={lon}&appid={API_KEY}"
    body = requests.get(url).json()
    return body


## ANCHOR: parse_air
def parse_air_pollution(body):
    res = []
    print(body)
    for row in body["list"]:
        res.append((row["main"]["aqi"], row["components"], row["dt"]))
    return res
    ## ANCHOR_END: parse_air


def print_air_pollution(main, components, dt):
    print("---")
    print(f"Air pollution forecast for {dt}")
    print(f"Air quality index: {main}")
    print("Components:")
    for k, v in components.items():
        print(f"  {k}: {v}")
    ## ANCHOR_END: forecast


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
    results = parse_air_pollution(body)

    for main, components, dt in results:
        print_air_pollution(main, components, dt)

## ANCHOR_END: all

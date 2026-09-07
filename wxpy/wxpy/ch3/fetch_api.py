import os
import sys

import requests

API_KEY = os.getenv("OWM_APPID")
URL = "https://api.openweathermap.org/data/2.5/air_pollution"


def get_air_pollution(lat, lon):
    params = {"lat": lat, "lon": lon, "appid": API_KEY}
    response = requests.get(URL, params=params, timeout=10)
    response.raise_for_status()
    return response.text


if __name__ == "__main__":
    usage = f"Usage: python {__file__} <lat> <lon>"

    if not API_KEY:
        print("Please set OWM_APPID environment variable")
        sys.exit(1)

    if len(sys.argv) != 3:
        print(usage)
        sys.exit(1)

    try:
        lat = float(sys.argv[1])
        lon = float(sys.argv[2])
    except ValueError:
        print(usage)
        sys.exit(1)

    try:
        body = get_air_pollution(lat, lon)
    except requests.RequestException as err:
        print(f"Request failed: {err}")
        sys.exit(1)

    print(body)

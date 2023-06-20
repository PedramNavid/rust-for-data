import os
import sys
import requests

API_KEY = os.getenv("OWM_APPID")


def get_air_pollution(lat, lon):
    url = f"http://api.openweathermap.org/data/2.5/air_pollution?lat={lat}&lon={lon}&appid={API_KEY}"
    body = requests.get(url).text
    return body


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
    print(body)

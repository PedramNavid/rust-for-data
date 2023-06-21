
## ANCHOR: all
import os
import json

API_KEY = os.getenv("OWM_APPID")


## ANCHOR: forecast
def get_air_pollution():
    script_path = os.path.dirname(os.path.realpath(__file__))
    with open(os.path.join(script_path, "../../../lib/big_payload.json"), "r") as f:
        return json.loads(f.read())



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
    body = get_air_pollution()
    results = parse_air_pollution(body)

    for main, components, dt in results:
        print_air_pollution(main, components, dt)

## ANCHOR_END: all

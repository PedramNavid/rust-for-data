import os
import requests

lat = 37.9871
lon = -122.5889
appid = os.getenv("OWM_APPID")

url = f"http://api.openweathermap.org/data/2.5/air_pollution?lat={lat}&lon={lon}&appid={appid}"

body = requests.get(url).text
print(body)

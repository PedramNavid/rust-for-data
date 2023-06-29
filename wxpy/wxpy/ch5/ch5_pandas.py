import os

import pandas as pd

script_path = os.path.dirname(os.path.realpath(__file__))
bird_path = os.path.join(script_path, "../../../lib/PFW_2016_2020_public.csv")
codes_path = os.path.join(script_path, "../../../lib/species_code.csv")

# adding usecols reducing memory usage and runtime from 13s to 7s
birds = pd.read_csv(
    bird_path,
    usecols=[
        "LATITUDE",
        "LONGITUDE",
        "SUBNATIONAL1_CODE",
        "Month",
        "Day",
        "Year",
        "SPECIES_CODE",
        "HOW_MANY",
        "VALID",
    ],
).rename(columns=lambda x: x.lower())

codes = pd.read_csv(codes_path)[["SPECIES_CODE", "PRIMARY_COM_NAME"]].rename(
    columns={"SPECIES_CODE": "species_code", "PRIMARY_COM_NAME": "species_name"}
)

birds = birds[
    [
        "latitude",
        "longitude",
        "subnational1_code",
        "month",
        "day",
        "year",
        "species_code",
        "how_many",
        "valid",
    ]
]

birds = birds[birds["valid"] == 1]
birds = (
    birds.groupby(["subnational1_code", "species_code"])
    .agg(total_species=("how_many", "sum"), total_sightings=("how_many", "count"))
    .reset_index()
    .sort_values("total_species", ascending=False)
)

birds = pd.merge(birds, codes, on="species_code", how="inner")


print(birds)

import pandas as pd
import os

script_path = os.path.dirname(os.path.realpath(__file__))
bird_path = os.path.join(script_path, "../../../lib/birds.csv")
codes_path = os.path.join(script_path, "../../../lib/species_code.csv")

birds = pd.read_csv(bird_path)
codes = pd.read_csv(codes_path)[["SPECIES_CODE", "PRIMARY_COM_NAME"]].rename(
    columns={"SPECIES_CODE": "species_code", "PRIMARY_COM_NAME": "species_name"}
)

birds_df = birds[
    [
        "latitude",
        "longitude",
        "subnational1_code",
        "Month",
        "Day",
        "Year",
        "species_code",
        "how_many",
        "valid",
    ]
]

birds_df = birds_df[birds_df["valid"] == 1]
birds_df = (
    birds_df.groupby(["subnational1_code", "species_code"])
    .agg(
        total_species=("how_many", "sum"),
        total_sightings=("how_many", "count"),
    )
    .reset_index()
    .sort_values("total_species", ascending=False)
)

birds_df = pd.merge(birds_df, codes, on="species_code", how="inner")


print(birds_df)

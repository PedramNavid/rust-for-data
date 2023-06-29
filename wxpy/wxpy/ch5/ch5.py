import os

import polars as pl

script_path = os.path.dirname(os.path.realpath(__file__))
bird_path = os.path.join(script_path, "../../../lib/PFW_2016_2020_public.csv")
codes_path = os.path.join(script_path, "../../../lib/species_code.csv")

columns = [
    "LATITUDE",
    "LONGITUDE",
    "SUBNATIONAL1_CODE",
    "Month",
    "Day",
    "Year",
    "SPECIES_CODE",
    "HOW_MANY",
    "VALID",
]

birds = pl.read_csv(
    bird_path,
    columns=columns,
    new_columns=[s.lower() for s in columns],
)

codes = pl.read_csv(codes_path).select(
    [
        pl.col("SPECIES_CODE").alias("species_code"),
        pl.col("PRIMARY_COM_NAME").alias("species_name"),
    ]
)

birds_df = (
    birds.select(
        pl.col(
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
        )
    )
    .filter(pl.col("valid") == 1)
    .groupby(["subnational1_code", "species_code"])
    .agg(
        [
            pl.sum("how_many").alias("total_species"),
            pl.count("how_many").alias("total_sightings"),
        ]
    )
    .sort("total_species", descending=True)
    .join(codes, on="species_code", how="inner")
)

print(birds_df)

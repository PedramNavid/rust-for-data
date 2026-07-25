import os

import polars as pl

script_path = os.path.dirname(os.path.realpath(__file__))
bird_path = os.path.join(script_path, "../../../lib/PFW_2016_2020_public.csv")
codes_path = os.path.join(script_path, "../../../lib/species_code.csv")

# The columns we care about, in the casing the CSV actually uses.
COLS = [
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

birds = pl.scan_csv(bird_path).select([pl.col(c).alias(c.lower()) for c in COLS])

codes = pl.scan_csv(codes_path, infer_schema_length=None).select(
    [
        pl.col("SPECIES_CODE").alias("species_code"),
        pl.col("PRIMARY_COM_NAME").alias("species_name"),
    ]
)

birds_df = (
    birds.filter(pl.col("valid") == 1)
    .group_by(["subnational1_code", "species_code"])
    .agg(
        [
            pl.col("how_many").sum().alias("total_species"),
            pl.col("how_many").count().alias("total_sightings"),
        ]
    )
    .join(codes, on="species_code", how="inner")
    .sort("total_species", descending=True)
    .collect()
)

print(birds_df)

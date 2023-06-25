import polars as pl
import os

script_path = os.path.dirname(os.path.realpath(__file__))
bird_path = os.path.join(script_path, "../../../lib/birds.csv")
codes_path = os.path.join(script_path, "../../../lib/species_code.csv")

birds = pl.read_csv(bird_path)
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
                "Month",
                "Day",
                "Year",
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

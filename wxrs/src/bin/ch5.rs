use polars::prelude::*;
use std::env;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let bird_path = current_dir.join("../lib/birds.csv");
    let codes_path = current_dir.join("../lib/species_code.csv");

    let birds_df = CsvReader::from_path(bird_path)
        .expect("Failed to read CSV file")
        .has_header(true)
        .finish()
        .unwrap();

    let mut codes_df = CsvReader::from_path(codes_path)
        .expect("Failed to read CSV file")
        .infer_schema(None)
        .has_header(true)
        .finish()
        .unwrap();

    codes_df = codes_df
        .clone()
        .lazy()
        .select([
            col("SPECIES_CODE").alias("species_code"),
            col("PRIMARY_COM_NAME").alias("species_name"),
        ])
        .collect()
        .unwrap();

    let birds_df = birds_df
        .lazy()
        .select([
            col("latitude"),
            col("longitude"),
            col("subnational1_code"),
            col("Month"),
            col("Day"),
            col("Year"),
            col("species_code"),
            col("how_many"),
            col("valid"),
        ])
        .filter(col("valid").eq(lit(1)))
        .groupby(["subnational1_code", "species_code"])
        .agg(&[
            col("how_many").sum().alias("total_species"),
            col("how_many").count().alias("total_sightings"),
        ])
        .sort(
            "total_species",
            SortOptions {
                descending: true,
                nulls_last: false,
                multithreaded: true,
            },
        )
        .collect()
        .unwrap();

    let joined = birds_df
        .join(
            &codes_df,
            ["species_code"],
            ["species_code"],
            JoinType::Inner,
            None,
        )
        .unwrap();

    println!("{}", joined);
}

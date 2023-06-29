use polars::prelude::*;
use std::env;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let bird_path = current_dir.join("../lib/PFW_2016_2020_public.csv");
    let codes_path = current_dir.join("../lib/species_code.csv");

    let cols = vec![
        "LATITUDE".into(),
        "LONGITUDE".into(),
        "SUBNATIONAL1_CODE".into(),
        "Month".into(),
        "Day".into(),
        "Year".into(),
        "SPECIES_CODE".into(),
        "HOW_MANY".into(),
        "VALID".into(),
    ];

    let birds_df = CsvReader::from_path(bird_path)
        .expect("Failed to read CSV file")
        .has_header(true)
        .with_columns(Some(cols.clone()))
        .finish()
        .unwrap()
        .lazy();

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
        .rename(cols.clone(), cols.into_iter().map(|x| x.to_lowercase()))
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

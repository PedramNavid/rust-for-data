use polars::prelude::*;

// Resolved at compile time relative to this crate, so the program works no
// matter which directory you run it from.
const BIRD_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../lib/PFW_2016_2020_public.csv"
);
const CODES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../lib/species_code.csv");

// The columns we care about, in the casing the CSV actually uses.
const COLS: [&str; 9] = [
    "LATITUDE",
    "LONGITUDE",
    "SUBNATIONAL1_CODE",
    "Month",
    "Day",
    "Year",
    "SPECIES_CODE",
    "HOW_MANY",
    "VALID",
];

fn main() -> PolarsResult<()> {
    let birds = LazyCsvReader::new(BIRD_PATH.into())
        .with_has_header(true)
        .finish()?
        .select(
            COLS.iter()
                .map(|name| col(*name).alias(name.to_lowercase()))
                .collect::<Vec<_>>(),
        );

    let codes = LazyCsvReader::new(CODES_PATH.into())
        .with_has_header(true)
        .with_infer_schema_length(None)
        .finish()?
        .select([
            col("SPECIES_CODE").alias("species_code"),
            col("PRIMARY_COM_NAME").alias("species_name"),
        ]);

    let joined = birds
        .filter(col("valid").eq(lit(1)))
        .group_by([col("subnational1_code"), col("species_code")])
        .agg([
            col("how_many").sum().alias("total_species"),
            col("how_many").count().alias("total_sightings"),
        ])
        .join(
            codes,
            [col("species_code")],
            [col("species_code")],
            JoinArgs::new(JoinType::Inner),
        )
        .sort(
            ["total_species"],
            SortMultipleOptions::default().with_order_descending(true),
        )
        .collect_with_engine(Engine::Streaming)?
        .unwrap_single();

    println!("{}", joined);
    Ok(())
}

/*Polars hello world */
use polars::prelude::*;

//

fn calculate() -> Result<DataFrame, PolarsError> {
    // Read Iris dataset from path
    let df = CsvReader::from_path("data/iris.csv")?
        .has_header(true)
        .finish()?
        .lazy()
        .collect()?;
    Ok(df)
}

fn main() {
    let calculated_df = calculate().unwrap();
    println!("Hello, world!");
    println!("{:?}", calculated_df);
    println!("{:?}", calculated_df.shape());
}

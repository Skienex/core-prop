use polars::prelude::{CsvReadOptions, DataFrame, PolarsResult, SerReader};

pub fn dataframe_from_csv(file_path: &str) -> PolarsResult<(DataFrame, DataFrame)> {
    let data = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(file_path.into()))?
        .finish()?;

    let training_dataset = data.drop("y")?;
    let training_labels = data.select(["y"])?;

    Ok((training_dataset, training_labels))
}
use ndarray::Array2;
use polars::prelude::{DataFrame, Float32Type, IndexOrder};

pub fn array_from_dataframe(df: &DataFrame) -> Array2<f32> {
    df.to_ndarray::<Float32Type>(IndexOrder::Fortran).unwrap().reversed_axes()
}
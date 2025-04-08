use ndarray::Array2;
use std::collections::HashMap;

pub struct NeuralNetwork {
    pub layers: Vec<usize>,
    pub learning_rate: f32,
}

impl NeuralNetwork {
    pub fn init_parameters_randomly(&self) -> HashMap<String, Array2<f32>> {
        todo!()
    }
}
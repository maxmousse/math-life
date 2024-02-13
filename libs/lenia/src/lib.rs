use maths::{
    convolution::{convolute, gaussian_kernel},
    function::normal_gauss,
    matrix::Matrix,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// Define the Lenia struct
pub struct Lenia {
    size: usize,
    time_constant: f64,
    state: Matrix<f64>,
    convolution_kernel: Matrix<f64>,
    growth_function: fn(&f64) -> f64,
}

#[wasm_bindgen]
impl Lenia {
    pub fn evolve(&mut self) {
        let result: Vec<f64> = self
            .state
            .iter()
            .enumerate()
            // Map index to coordinates
            .map(|(index, _)| self.state.index_to_coordinate(index))
            // Convolution
            .map(|point| {
                (
                    point,
                    convolute(&point, &self.state, &self.convolution_kernel),
                )
            })
            // Apply growth function
            .map(|(point, convolution_result)| {
                self.state.get_by_coordinate(&point)
                    + (1.0 / self.time_constant) * &(self.growth_function)(&convolution_result)
            })
            // clamp
            .map(|val| val.clamp(0.0, 1.0)) // clamp
            .collect();

        self.state = Matrix::from_vec(result, self.size, self.size).unwrap();
    }

    pub fn state(&self) -> *const f64 {
        self.state.m.as_ptr()
    }
}

impl Lenia {
    pub fn new(
        size: usize,
        time_constant: f64,
        growth_function: fn(&f64) -> f64,
        convolution_kernel: Matrix<f64>,
    ) -> Self {
        Self {
            size,
            time_constant,
            convolution_kernel,
            growth_function,
            state: Matrix::from_constant(size, size, 0.0),
        }
    }
}

#[wasm_bindgen]
pub fn lenia() -> Lenia {
    Lenia::new(
        64,
        10.0,
        |x| normal_gauss(*x, 0.135, 0.015),
        gaussian_kernel(10, 0.5, 0.15),
    )
}

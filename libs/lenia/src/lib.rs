use maths::{
    convolution::{gaussian_kernel, toroidal_convolute},
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
        // Convolve the grid with the convolution kernel
        let convolution_result = toroidal_convolute(&self.state, &self.convolution_kernel);

        for y in 0..self.size {
            for x in 0..self.size {
                // n+1 = (n + (1 / dt) * growth(convolute(n) ).clamp(0,1)
                self.state.m[y][x] = (self.state.m[y][x]
                    + (1.0 / self.time_constant)
                        * &(self.growth_function)(&convolution_result.m[y][x]))
                    .clamp(0.0, 1.0);
            }
        }
    }

    pub fn state(&self) -> &Vec<Vec<f64>> {
        &self.state.m
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

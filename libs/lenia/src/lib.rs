use maths::{
    convolution::{convolute, gaussian_kernel}, coordinate::Coordinate, function::gauss, matrix::Matrix
};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
// Define the Lenia struct
pub struct Lenia {
    size: usize,
    time_constant: f64,
    convoluted_state: Matrix<f64>,
    state: Matrix<f64>,
    convolution_kernel: Matrix<f64>,
    reversed_convolution_kernel: Matrix<f64>,
    growth_function: fn(&f64) -> f64,
}

#[wasm_bindgen]
impl Lenia {
    pub fn evolve(&mut self) {
        // Convolution
        for y in 0..self.size {
          for x in 0..self.size {
            let point = Coordinate(x, y);
            self.convoluted_state.set(&point, convolute(&point, &self.state, &self.reversed_convolution_kernel));
          }
        }

            // Apply growth function
        for y in 0..self.size {
          for x in 0..self.size {
            let point = Coordinate(x, y);
            let current_state = self.state.get_by_coordinate(&point);
            let convoluted_state = self.convoluted_state.get_by_coordinate(&point);
            let next_state = (current_state + (1.0 / self.time_constant) * &(self.growth_function)(convoluted_state)).clamp(0.0, 1.0);
            self.state.set(&point, next_state);
          }
        }

        // let result: Vec<f64> = self
        //     .state
        //     .iter()
        //     .enumerate()
        //     // Map index to coordinates
        //     .map(|(index, _)| self.state.index_to_coordinate(index))
        //     .map(|point| {
        //         (
        //             point,
        //             convolute(&point, &self.state, &self.reversed_convolution_kernel),
        //         )
        //     })
        //     // Apply growth function
        //     .map(|(point, convolution_result)| {
        //         self.state.get_by_coordinate(&point)
        //             + (1.0 / self.time_constant) * &(self.growth_function)(&convolution_result)
        //     })
        //     // clamp
        //     .map(|val| val.clamp(0.0, 1.0)) // clamp
        //     .collect();

        // self.state = Matrix::from_vec(result, self.size, self.size).unwrap();
    }

    pub fn state(&self) -> *const f64 {
        self.state.m.as_ptr()
    }

    pub fn convoluted_state(&self) -> *const f64 {
        self.convoluted_state.m.as_ptr()
    }

    pub fn convolution_kernel(&self) -> *const f64 {
        self.convolution_kernel.m.as_ptr()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Lenia {
    pub fn new(
        size: usize,
        time_constant: f64,
        growth_function: fn(&f64) -> f64,
        convolution_kernel: Matrix<f64>,
    ) -> Self {
        set_panic_hook();
        let orbium: Vec<Vec<f64>> = vec![
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.1, 0.14, 0.1, 0.0, 0.0, 0.03, 0.03, 0.0, 0.0, 0.3,
                0.0, 0.0, 0.0, 0.0,
            ],
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.08, 0.24, 0.3, 0.3, 0.18, 0.14, 0.15, 0.16, 0.15, 0.09,
                0.2, 0.0, 0.0, 0.0, 0.0,
            ],
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.15, 0.34, 0.44, 0.46, 0.38, 0.18, 0.14, 0.11, 0.13,
                0.19, 0.18, 0.45, 0.0, 0.0, 0.0,
            ],
            vec![
                0.0, 0.0, 0.0, 0.0, 0.06, 0.13, 0.39, 0.5, 0.5, 0.37, 0.06, 0.0, 0.0, 0.0, 0.02,
                0.16, 0.68, 0.0, 0.0, 0.0,
            ],
            vec![
                0.0, 0.0, 0.0, 0.11, 0.17, 0.17, 0.33, 0.4, 0.38, 0.28, 0.14, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.18, 0.42, 0.0, 0.0,
            ],
            vec![
                0.0, 0.0, 0.09, 0.18, 0.13, 0.06, 0.08, 0.26, 0.32, 0.32, 0.27, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.82, 0.0, 0.0,
            ],
            vec![
                0.27, 0.0, 0.16, 0.12, 0.0, 0.0, 0.0, 0.25, 0.38, 0.44, 0.45, 0.34, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.22, 0.17, 0.0,
            ],
            vec![
                0.0, 0.07, 0.2, 0.02, 0.0, 0.0, 0.0, 0.31, 0.48, 0.57, 0.6, 0.57, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.49, 0.0,
            ],
            vec![
                0.0, 0.59, 0.19, 0.0, 0.0, 0.0, 0.0, 0.2, 0.57, 0.69, 0.76, 0.76, 0.49, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.36, 0.0,
            ],
            vec![
                0.0, 0.58, 0.19, 0.0, 0.0, 0.0, 0.0, 0.0, 0.67, 0.83, 0.9, 0.92, 0.87, 0.12, 0.0,
                0.0, 0.0, 0.0, 0.22, 0.07,
            ],
            vec![
                0.0, 0.0, 0.46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.7, 0.93, 1.0, 1.0, 1.0, 0.61, 0.0, 0.0,
                0.0, 0.0, 0.18, 0.11,
            ],
            vec![
                0.0, 0.0, 0.82, 0.0, 0.0, 0.0, 0.0, 0.0, 0.47, 1.0, 1.0, 0.98, 1.0, 0.96, 0.27,
                0.0, 0.0, 0.0, 0.19, 0.1,
            ],
            vec![
                0.0, 0.0, 0.46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.25, 1.0, 1.0, 0.84, 0.92, 0.97, 0.54,
                0.14, 0.04, 0.1, 0.21, 0.05,
            ],
            vec![
                0.0, 0.0, 0.0, 0.4, 0.0, 0.0, 0.0, 0.0, 0.09, 0.8, 1.0, 0.82, 0.8, 0.85, 0.63,
                0.31, 0.18, 0.19, 0.2, 0.01,
            ],
            vec![
                0.0, 0.0, 0.0, 0.36, 0.1, 0.0, 0.0, 0.0, 0.05, 0.54, 0.86, 0.79, 0.74, 0.72, 0.6,
                0.39, 0.28, 0.24, 0.13, 0.0,
            ],
            vec![
                0.0, 0.0, 0.0, 0.01, 0.3, 0.07, 0.0, 0.0, 0.08, 0.36, 0.64, 0.7, 0.64, 0.6, 0.51,
                0.39, 0.29, 0.19, 0.04, 0.0,
            ],
            vec![
                0.0, 0.0, 0.0, 0.0, 0.1, 0.24, 0.14, 0.1, 0.15, 0.29, 0.45, 0.53, 0.52, 0.46, 0.4,
                0.31, 0.21, 0.08, 0.0, 0.0,
            ],
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.08, 0.21, 0.21, 0.22, 0.29, 0.36, 0.39, 0.37, 0.33,
                0.26, 0.18, 0.09, 0.0, 0.0, 0.0,
            ],
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.03, 0.13, 0.19, 0.22, 0.24, 0.24, 0.23, 0.18, 0.13,
                0.05, 0.0, 0.0, 0.0, 0.0,
            ],
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.02, 0.06, 0.08, 0.09, 0.07, 0.05, 0.01,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        ];

        let mut reversed_convolution_kernel = convolution_kernel.clone();
        reversed_convolution_kernel.m.reverse();

        Self {
            size,
            time_constant,
            convolution_kernel,
            reversed_convolution_kernel,
            growth_function,
            convoluted_state: Matrix::from_constant(size, size, 0.0),
            state: Matrix::from_function(size, size, |x, y| {
                *orbium.get(y).and_then(|row| row.get(x)).unwrap_or(&0.0)
            }),
        }
    }
}

#[wasm_bindgen]
pub fn lenia() -> Lenia {
    let kernel = gaussian_kernel(13, 0.5, 0.15);
    Lenia::new(
        64,
        10.0,
        |x| gauss(*x, 2.0, 0.15, 0.015) - 1.0,
        kernel,
    )
}

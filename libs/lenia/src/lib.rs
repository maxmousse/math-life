use maths::matrix::Matrix;

// Define the Lenia struct
pub struct Lenia {
    size: usize,
    time_constant: f64,
    grid: Matrix<f64>,
    convolution_kernel: Matrix<f64>,
    growth_function: fn(f64) -> f64,
}

impl Lenia {
    pub fn new(
        size: usize,
        time_constant: f64,
        growth_function: fn(f64) -> f64,
        convolution_kernel: Matrix<f64>,
    ) -> Self {
        Self {
            size,
            time_constant,
            convolution_kernel,
            growth_function,
            grid: Matrix::from_constant(size, size, 0.0),
        }
    }
    // evolve
}

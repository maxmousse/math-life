// Define the Lenia struct
type Matrix<T> = Vec<Vec<T>>;

pub struct Lenia {
    size: usize,
    time_constant: f64,
    radius: usize,
    grid: Matrix<f64>,
    convolution_kernel: Matrix<f64>,
}

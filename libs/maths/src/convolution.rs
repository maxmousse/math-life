use crate::{
    function::{distance, normal_gauss, toroidal_point_translation},
    matrix::Matrix,
};

pub fn toroidal_convolute(matrix: &Matrix<f64>, kernel: &Matrix<f64>) -> Matrix<f64> {
    // TODO: check that kernel is smaller than matrix
    let mut result = Matrix::from_constant(matrix.width, matrix.height, 0.0);

    // Iterate through the matrix
    for (m_y, m_row) in matrix.m.iter().enumerate() {
        for (m_x, m_val) in m_row.iter().enumerate() {
            // For each matrix cell, convolute
            result.m[m_y][m_x] = convolute_one((m_x, m_y), matrix, kernel);
        }
    }

    result
}

fn convolute_one(point: (usize, usize), matrix: &Matrix<f64>, kernel: &Matrix<f64>) -> f64 {
    let mut result = 0.0;

    for (k_y, k_row) in kernel.m.iter().enumerate() {
        for (k_x, k_coef) in k_row.iter().enumerate() {
            let (x, y) =
                toroidal_point_translation(point, (k_x, k_y), (matrix.width, matrix.height));
            result += k_coef * matrix.m[y][x];
        }
    }

    result
}

/// Generate a normalized gaussian kernel
///
/// A normalized Gaussian kernel is a two-dimensional matrix representing
/// a Gaussian distribution. The Gaussian kernel is often used as a smoothing filter in image
/// processing and computer vision.
pub fn gaussian_kernel(radius: usize, mean: f64, standard_deviation: f64) -> Matrix<f64> {
    let mut kernel = distance_kernel(radius);
    kernel.map(|val| {
        if *val != 0.0 {
            normal_gauss(*val, mean, standard_deviation)
        } else {
            *val
        }
    });
    let sum = kernel.sum();
    kernel.map(|val| val / sum);
    kernel
}

/// Generate a normalized distance kernel of the specified radius
///
/// A distance kernel is a squared matrix of size `radius * 2 + 1`.
/// Each element of the matrix represents the distance of the corresponding
/// point in the convolution kernel from the center, and normalizing ensures
/// that the values are within a specific range
pub fn distance_kernel(radius: usize) -> Matrix<f64> {
    // Matrix size must be odd to have a center
    let diameter = (radius * 2) + 1;

    Matrix::from_function(diameter, diameter, |x, y|
        // divide by `radius` for normalization
        distance((x as f64, y as f64), (radius as f64, radius as f64)) / radius as f64)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance_kernel() {
        let expected_result = vec![
            vec![2.0_f64.sqrt(), 1.0, 2.0_f64.sqrt()],
            vec![1.0, 0.0, 1.0],
            vec![2.0_f64.sqrt(), 1.0, 2.0_f64.sqrt()],
        ];

        assert_eq!(
            distance_kernel(1),
            Matrix::from_vec(expected_result).unwrap()
        );
    }
}

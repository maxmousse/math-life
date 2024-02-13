use crate::{
    coordinate::{toroidal_translation, Coordinate},
    function::{distance, normal_gauss},
    matrix::Matrix,
};

// pub fn toroidal_convolute(matrix: &Matrix<f64>, kernel: &Matrix<f64>) -> Matrix<f64> {
//     // TODO: check that kernel is smaller than matrix
//     let mut result = Matrix::from_constant(matrix.width, matrix.height, 0.0);
//
//     // Iterate through the matrix
//     for (m_y, m_row) in matrix.m.iter().enumerate() {
//         for (m_x, m_val) in m_row.iter().enumerate() {
//             // For each matrix cell, convolute
//             result.m[m_y][m_x] = convolute_one((m_x, m_y), matrix, kernel);
//         }
//     }
//
//     result
// }

pub fn convolute(point: &Coordinate, matrix: &Matrix<f64>, kernel: &Matrix<f64>) -> f64 {
    kernel
        .iter()
        .enumerate()
        .map(|(k_index, k_val)| (kernel.index_to_coordinate(k_index), k_val))
        .fold(0.0, |result, (k_coordinate, k_coef)| {
            let neighbor_coordinates =
                toroidal_translation(point, &k_coordinate, &matrix.width, &matrix.height);

            result + k_coef * matrix.get_by_coordinate(&neighbor_coordinates)
        })
}

/// Generate a normalized gaussian kernel
///
/// A normalized Gaussian kernel is a two-dimensional matrix representing
/// a Gaussian distribution. The Gaussian kernel is often used as a smoothing filter in image
/// processing and computer vision.
pub fn gaussian_kernel(radius: usize, mean: f64, standard_deviation: f64) -> Matrix<f64> {
    let mut kernel = distance_kernel(radius);

    // Turn distance kernel into gaussian kernel
    kernel.iter_mut().for_each(|val| {
        if *val != 0.0 {
            *val = normal_gauss(*val, mean, standard_deviation)
        }
    });

    // Normalize the kernel
    let sum = kernel.iter().fold(0.0, |sum, val| sum + val);
    kernel.iter_mut().for_each(|val| *val = *val / sum);

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
            2.0_f64.sqrt(),
            1.0,
            2.0_f64.sqrt(),
            1.0,
            0.0,
            1.0,
            2.0_f64.sqrt(),
            1.0,
            2.0_f64.sqrt(),
        ];

        assert_eq!(
            distance_kernel(1),
            Matrix::from_vec(expected_result, 3, 3).unwrap()
        );
    }
}

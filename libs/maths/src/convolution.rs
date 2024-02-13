use crate::{
    coordinate::{toroidal_translation, vector, Coordinate},
    function::{distance, normal_gauss},
    matrix::Matrix,
};

pub fn convolute(point: &Coordinate, matrix: &Matrix<f64>, kernel: &Matrix<f64>) -> f64 {
    let k_center = kernel.height / 2;

    kernel
        .iter()
        .enumerate()
        // Get current kernel cell coordinates
        .map(|(k_index, k_val)| (kernel.index_to_coordinate(k_index), k_val))
        // Get the vector from kernel center to current kernel cell
        .map(|(k_coordinate, k_val)| {
            (
                vector(&Coordinate(k_center, k_center), &k_coordinate),
                k_val,
            )
        })
        .fold(0.0, |result, (k_vector, k_coef)| {
            let neighbor_coordinates =
                toroidal_translation(point, &k_vector, &matrix.width, &matrix.height);

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
    fn test_convolute() {
        let matrix =
            Matrix::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 3, 3).unwrap();
        let kernel =
            Matrix::from_vec(vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0], 3, 3).unwrap();

        assert_eq!(convolute(&Coordinate(1, 1), &matrix, &kernel), 25.0);
    }

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

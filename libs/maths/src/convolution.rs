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

            let neighbor_val = matrix.get_by_coordinate(&neighbor_coordinates);
            //println!("point {:?}", point);
            //println!("k_vecotr {:?}", k_vector);
            //println!("k_coef {:?}", k_coef);
            //println!("neighbor {:?}", neighbor_coordinates);
            //println!("neighbor val{:?}", neighbor_val);
            //println!("");

            result + k_coef * neighbor_val
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
    let diameter = radius * 2;
    let center = (radius - 1) as f64;

    Matrix::from_function(diameter, diameter, |x, y|
        // divide by `radius` for normalization
        distance((x as f64, y as f64), (center, center)) / radius as f64)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convolute() {
        let input = Matrix::from_function(3, 3, |x, y| ((y * 3) + x + 1) as f64);
        let mut kernel =
            Matrix::from_function(3, 3, |x, y| if x == 1 && y == 1 { 0.0 } else { 1.0 });
        kernel.m.reverse();

        println!("input: {:?}", input);
        println!("kernel: {:?}", kernel);

        let expected_result: Vec<f64> = vec![
            vec![44.0, 38.0, 44.0, 38.0],
            vec![36.0, 30.0, 36.0, 30.0],
            vec![60.0, 54.0, 60.0, 54.0],
            vec![52.0, 46.0, 52.0, 46.0],
        ]
        .into_iter()
        .flatten()
        .collect();

        let result: Vec<f64> = input
            .iter()
            .enumerate()
            .map(|(index, _)| input.index_to_coordinate(index))
            // Convolution
            .map(|point| convolute(&point, &input, &kernel))
            .collect();

        println!("result: {:?}", result);

        // assert_eq!(convolute(&Coordinate(1, 1), &input, &kernel), 30.0);
        assert_eq!(
            Matrix::from_vec(result, 4, 4).unwrap(),
            Matrix::from_vec(expected_result, 4, 4).unwrap()
        );
    }

    #[test]
    fn test_convolute_gaussian_kernel() {
        // import numpy as np
        // import scipy as sp
        //
        // def gauss(x, mu, sigma):
        //     return np.exp(-0.5 * ((x-mu)/sigma)**2)
        //
        // R = 2
        // D = R * 2
        // y, x = np.ogrid[-R:R, -R:R]
        // distance = np.sqrt((1+x)**2 + (1+y)**2) / R
        //
        // mu = 0.5
        // sigma = 0.15
        // K_lenia = gauss(distance, mu, sigma)
        // K_lenia[distance > 1] = 0               # Cut at d=1
        // K_lenia = K_lenia / np.sum(K_lenia)     # Normalize
        //
        // print(K_lenia)
        //
        // X =  np.arange(D * D).reshape(D, D)
        //
        // print(X)
        //
        // U = sp.signal.convolve2d(X, K_lenia, mode='same', boundary='wrap')
        //
        // print(U)
        //
        // print(type(U))

        let input = Matrix::from_function(4, 4, |x, y| ((y * 4) + x) as f64);
        let mut kernel = gaussian_kernel(2, 0.5, 0.15);
        kernel.m.reverse();

        let expected_result: Vec<f64> = vec![
            vec![6.38484025, 6.10926441, 7.10647999, 6.83090415],
            vec![5.28253689, 5.00696105, 6.00417663, 5.72860079],
            vec![9.27139921, 8.99582337, 9.99303895, 9.71746311],
            vec![8.16909585, 7.89352001, 8.89073559, 8.61515975],
        ]
        .into_iter()
        .flatten()
        .collect();

        let result: Vec<f64> = input
            .iter()
            .enumerate()
            .map(|(index, _)| input.index_to_coordinate(index))
            // Convolution
            .map(|point| convolute(&point, &input, &kernel))
            .collect();

        //assert_eq!(convolute(&Coordinate(1, 1), &input, &kernel), 5.00696105);
        assert_eq!(
            Matrix::from_vec(result, 4, 4).unwrap(),
            Matrix::from_vec(expected_result, 4, 4).unwrap()
        );
    }

    #[test]
    fn test_distance_kernel() {
        // Note : expected result is the result of the
        // original python code:
        //
        // ```python
        // R = 2
        // y, x = np.ogrid[-R:R, -R:R]
        // distance = np.sqrt((1+x)**2 + (1+y)**2) / R
        // ```
        let expected_result: Vec<f64> = vec![
            vec![
                0.7071067811865476,
                0.5,
                0.7071067811865476,
                1.118033988749895,
            ],
            vec![0.5, 0.0, 0.5, 1.0],
            vec![
                0.7071067811865476,
                0.5,
                0.7071067811865476,
                1.118033988749895,
            ],
            vec![
                1.118033988749895,
                1.0,
                1.118033988749895,
                1.4142135623730951,
            ],
        ]
        .into_iter()
        .flatten()
        .collect();

        assert_eq!(
            distance_kernel(2),
            Matrix::from_vec(expected_result, 4, 4).unwrap()
        );
    }

    #[test]
    fn test_gaussian_kernel() {
        // Note : expected result is the result of the
        // original python code:
        //
        // ```python
        // def gauss(x, mu, sigma):
        // return np.exp(-0.5 * ((x-mu)/sigma)**2)
        //
        // R = 2
        // y, x = np.ogrid[-R:R, -R:R]
        // distance = np.sqrt((1+x)**2 + (1+y)**2) / R
        //
        // mu = 0.5
        // sigma = 0.15
        // K_lenia = gauss(distance, mu, sigma)
        // K_lenia[distance > 1] = 0               # Cut at d=1
        // K_lenia = K_lenia / np.sum(K_lenia)     # Normalize
        // ```
        let expected_result: Vec<f64> = vec![
            vec![
                0.06945408599250456,
                0.18016057586483494,
                0.06945408599250456,
                3.7094556619432716e-5,
            ],
            vec![
                0.18016057586483494,
                0.0,
                0.18016057586483494,
                0.0006964863985748835,
            ],
            vec![
                0.06945408599250456,
                0.18016057586483494,
                0.06945408599250456,
                3.7094556619432716e-5,
            ],
            vec![
                3.7094556619432716e-5,
                0.0006964863985748835,
                3.7094556619432716e-5,
                1.5470148210470049e-9,
            ],
        ]
        .into_iter()
        .flatten()
        .collect();

        assert_eq!(
            gaussian_kernel(2, 0.5, 0.15),
            Matrix::from_vec(expected_result, 4, 4).unwrap()
        );
    }
}

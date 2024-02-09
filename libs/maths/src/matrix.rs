use std::ops::{Add, AddAssign, Sub};

/// Simple 2 dimensions matrix struct
#[derive(Debug, PartialEq)]
pub struct Matrix<T: Copy + Add + Sub<Output = T>> {
    pub width: usize,
    pub height: usize,
    pub m: Vec<Vec<T>>,
}

impl<T: Copy + Add + AddAssign + Sub<Output = T>> Matrix<T> {
    /// Instantiate a matrix of size `width` * `height` filled with the
    /// specified constant value
    pub fn from_constant(width: usize, height: usize, constant: T) -> Self {
        Self {
            width,
            height,
            m: vec![vec![constant; width]; height],
        }
    }

    /// Instantiate a matrix of size `width` * `height` filled with the
    /// result of the specified closure
    ///
    /// Closure will be called with the coordinate of each matrix cell
    pub fn from_function<F>(width: usize, height: usize, f: F) -> Self
    where
        F: Fn(usize, usize) -> T,
    {
        let m = (0..height)
            .map(|y| (0..width).map(|x| f(x, y)).collect())
            .collect();
        Self { width, height, m }
    }

    /// Instantiate a matrix from a 2 dimension vector
    ///
    /// For the vector to be a valid matrix, it should:
    /// - have 2 dimensions
    /// - each dimension must have a non null size
    /// - the size of the dimensions should be homogenous
    pub fn from_vec(src: Vec<Vec<T>>) -> Result<Self, String> {
        let height = src.len();

        if height == 0 {
            return Err("Matrix height can not be null".to_string());
        }

        let width = src.first().unwrap().len();

        if width == 0 {
            return Err("Matrix width can not be null".to_string());
        }

        if src.iter().any(|row| row.len() != width) {
            return Err("Matrix width should be homogenous".to_string());
        }

        Ok(Self {
            height,
            width,
            m: src,
        })
    }

    /// Edit the matrix instance in place by applying `f` to each cell
    pub fn map<F>(&mut self, f: F)
    where
        F: Fn(&T) -> T,
    {
        self.m
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|val| *val = f(val)));
    }

    // Sum all the matrix cells
    pub fn sum(&self) -> T {
        let init = self.m[0][0];
        let result = self.m.iter().fold(init, |mut sum, row| {
            row.iter().for_each(|val| sum += *val);
            sum
        });
        result - init
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_constant() {
        let size = 3;
        let expected_result = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];

        assert_eq!(Matrix::from_constant(size, size, 1).m, expected_result);
    }

    #[test]
    fn test_from_function() {
        let size = 3;
        let expected_result = vec![vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4]];

        assert_eq!(
            Matrix::from_function(size, size, |x, y| x + y).m,
            expected_result
        );
    }

    #[test]
    fn test_from_vec_valid_matrix() {
        let input_matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let result = Matrix::from_vec(input_matrix.clone()).unwrap();
        assert_eq!(result.width, 3);
        assert_eq!(result.height, 3);
        assert_eq!(result.m, input_matrix);
    }

    #[test]
    fn test_from_vec() {
        // Matrix height error
        let mut input_matrix = vec![];
        let mut result = Matrix::from_vec(input_matrix);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Matrix height can not be null");

        // Matrix width error
        input_matrix = vec![vec![], vec![], vec![]];
        result = Matrix::from_vec(input_matrix);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Matrix width can not be null");

        // Matrix homogeneity error
        input_matrix = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8]];
        result = Matrix::from_vec(input_matrix);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Matrix width should be homogenous");

        // Valid matrix
        input_matrix = vec![vec![1, 2, 3], vec![4, 5, 7], vec![8, 9, 10]];
        result = Matrix::from_vec(input_matrix.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().m, input_matrix);
    }

    #[test]
    fn test_map() {
        let mut matrix = Matrix::from_vec(vec![vec![1, 2], vec![3, 4]]).unwrap();

        // Map each element to its square
        matrix.map(|val| val * 2);

        let expected_result = vec![vec![2, 4], vec![6, 8]];

        assert_eq!(matrix.m, expected_result);
    }
}

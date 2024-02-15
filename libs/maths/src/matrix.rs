use std::iter::Iterator;
use std::ops::{Add, AddAssign, Sub};

use crate::coordinate::Coordinate;
/// Simple 2 dimensions matrix struct
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T: Copy + Add + Sub<Output = T>> {
    pub width: usize,
    pub height: usize,
    pub m: Vec<T>,
}

impl<T: Copy + Add + AddAssign + Sub<Output = T>> Matrix<T> {
    /// Instantiate a matrix of size `width` * `height` filled with the
    /// specified constant value
    pub fn from_constant(width: usize, height: usize, constant: T) -> Self {
        Self {
            width,
            height,
            m: vec![constant; width * height],
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
        let mut m = Vec::new();
        (0..height).for_each(|y| (0..width).for_each(|x| m.push(f(x, y))));
        Self { width, height, m }
    }

    /// Instantiate a matrix from a 2 dimension vector
    ///
    /// For the vector to be a valid matrix, it should:
    /// - have 2 dimensions
    /// - each dimension must have a non null size
    /// - the size of the dimensions should be homogenous
    pub fn from_vec(data: Vec<T>, width: usize, height: usize) -> Result<Self, String> {
        if height == 0 {
            return Err("Matrix height can not be null".to_string());
        }

        if width == 0 {
            return Err("Matrix width can not be null".to_string());
        }

        if data.len() != height * width {
            return Err("Data size does not match matrix size".to_string());
        }

        Ok(Self {
            height,
            width,
            m: data,
        })
    }

    /// Get the value of a matrix cell by coordinate
    pub fn get_by_coordinate(&self, coordinate: &Coordinate) -> &T {
        &self.m[self.coordinate_to_index(coordinate)]
    }

    /// Get the value of a matrix cell by index
    pub fn get_by_index(&self, index: usize) -> &T {
        &self.m[index]
    }

    /// Turn an index into the equivalent coordinate for the current matrix
    pub fn index_to_coordinate(&self, index: usize) -> Coordinate {
        let y = index / self.width;
        let x = index % self.width;

        Coordinate(x, y)
    }

    /// Turn a coordinate into the equivalent index for the current matrix
    pub fn coordinate_to_index(&self, Coordinate(x, y): &Coordinate) -> usize {
        y * self.width + x
    }

    pub fn into_iter(self) -> std::vec::IntoIter<T> {
        self.m.into_iter()
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.m.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T> {
        self.m.iter_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_constant() {
        let size = 3;
        let expected_result = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];

        assert_eq!(Matrix::from_constant(size, size, 1).m, expected_result);
    }

    #[test]
    fn test_from_function() {
        let size = 3;
        let expected_result = vec![0, 1, 2, 1, 2, 3, 2, 3, 4];

        assert_eq!(
            Matrix::from_function(size, size, |x, y| x + y).m,
            expected_result
        );
    }

    #[test]
    fn test_from_vec() {
        // Matrix height error
        let mut data: Vec<u32> = vec![];
        let mut result = Matrix::from_vec(data, 0, 1);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Matrix width can not be null");

        // Matrix width error
        data = vec![];
        result = Matrix::from_vec(data, 10, 0);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Matrix height can not be null");

        // Matrix homogeneity error
        data = vec![1, 2, 3];
        result = Matrix::from_vec(data, 2, 2);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Data size does not match matrix size"
        );

        // Valid matrix
        data = vec![1, 2, 3, 4];
        result = Matrix::from_vec(data.clone(), 2, 2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().m, data);
    }

    #[test]
    fn test_index_to_coordinate() {
        let matrix = Matrix::from_constant(3, 3, 0);

        let result: Vec<Coordinate> = matrix
            .iter()
            .enumerate()
            .map(|(index, _)| matrix.index_to_coordinate(index))
            .collect();

        let expected_result = vec![
            Coordinate(0, 0),
            Coordinate(1, 0),
            Coordinate(2, 0),
            Coordinate(0, 1),
            Coordinate(1, 1),
            Coordinate(2, 1),
            Coordinate(0, 2),
            Coordinate(1, 2),
            Coordinate(2, 2),
        ];

        assert_eq!(result, expected_result);
    }
}

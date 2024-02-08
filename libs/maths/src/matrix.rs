pub struct Matrix<T: Copy> {
    width: usize,
    height: usize,
    m: Vec<Vec<T>>,
}

impl<T: Copy> Matrix<T> {
    pub fn from_constant(width: usize, height: usize, constant: T) -> Self {
        Self {
            width,
            height,
            m: vec![vec![constant; width]; height],
        }
    }

    pub fn from_function<F>(width: usize, height: usize, function: F) -> Self
    where
        F: Fn(usize, usize) -> T,
    {
        let m = (0..height)
            .map(|y| (0..width).map(|x| function(x, y)).collect())
            .collect();
        Self { width, height, m }
    }
}

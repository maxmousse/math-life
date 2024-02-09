pub fn gauss(x: f64, height: f64, mean: f64, standard_deviation: f64) -> f64 {
    height * (-((x - mean) / standard_deviation).powi(2) / 2.0).exp()
}

pub fn normal_gauss(x: f64, mean: f64, standard_deviation: f64) -> f64 {
    gauss(x, 1.0, mean, standard_deviation)
}

/// Calculate the distance between two points in a plan
///
/// √[(x2 − x1)2 + (y2 − y1)2]
pub fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt()
}

pub fn toroidal_point_translation(
    point: (usize, usize),
    vector: (usize, usize),
    plan_dimension: (usize, usize),
) -> (usize, usize) {
    let (point_x, point_y) = point;
    let (vector_x, vector_y) = vector;
    let (plan_width, plan_height) = plan_dimension;

    (
        (point_x + vector_x) % plan_width,
        (point_y + vector_y) % plan_height,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance() {
        // Distance from self should be 0
        assert_eq!(distance((0.0, 0.0), (0.0, 0.0)), 0.0);

        assert_eq!(distance((0.0, 0.0), (1.0, 0.0)), 1.0);
        assert_eq!(distance((0.0, 0.0), (1.0, 1.0)), 2.0_f64.sqrt());
    }
}

pub fn gauss(x: f64, height: f64, center: f64, standard_deviation: f64) -> f64 {
    height * (-((x - center) / standard_deviation).powi(2) / 2.0).exp()
}

pub fn normal_gauss(x: f64, center: f64, standard_deviation: f64) -> f64 {
    gauss(x, 1.0, center, standard_deviation)
}

/// Calculate the distance between two points in a plan
///
/// √[(x2 − x1)2 + (y2 − y1)2]
pub fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt()
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

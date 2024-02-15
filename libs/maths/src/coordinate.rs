/// Represents coordinates of a point in a
/// plan. Coordinates can not be negative
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate(pub usize, pub usize);

/// Represents a 2 dimension vector. Its component can be negative
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector(pub isize, pub isize);

/// Determine the vector to go from a source point to
/// a destination point
pub fn vector(source: &Coordinate, destination: &Coordinate) -> Vector {
    let Coordinate(source_x, source_y) = source;
    let Coordinate(destination_x, destination_y) = destination;

    Vector(
        *destination_x as isize - *source_x as isize,
        *destination_y as isize - *source_y as isize,
    )
}

/// Apply a translation of a point by a vector
/// in a toroidal (periodic) plan
pub fn toroidal_translation(
    point: &Coordinate,
    vector: &Vector,
    plan_width: &usize,
    plan_height: &usize,
) -> Coordinate {
    let Coordinate(point_x, point_y) = point;
    let Vector(vector_x, vector_y) = vector;
    let width = *plan_width as isize;
    let height = *plan_height as isize;

    let result_x = ((*point_x as isize + vector_x + width) % width) as usize;
    let result_y = ((*point_y as isize + vector_y + height) % height) as usize;

    Coordinate(result_x, result_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let data = [
            // Test x axis
            (Coordinate(0, 0), Coordinate(1, 0), Vector(1, 0)),
            (Coordinate(1, 0), Coordinate(0, 0), Vector(-1, 0)),
            // Test y axis
            (Coordinate(0, 0), Coordinate(0, 1), Vector(0, 1)),
            (Coordinate(0, 1), Coordinate(0, 0), Vector(0, -1)),
            // Test x and y axis
            (Coordinate(0, 0), Coordinate(1, 1), Vector(1, 1)),
            (Coordinate(1, 1), Coordinate(0, 0), Vector(-1, -1)),
        ];

        data.iter().for_each(|(source, destination, result)| {
            assert_eq!(vector(&source, &destination), *result)
        });
    }

    #[test]
    fn test_toroidal_translation() {
        let data = [
            // Test x toroidal translation within bounds
            (Coordinate(0, 0), Vector(1, 0), Coordinate(1, 0)),
            (Coordinate(1, 0), Vector(-1, 0), Coordinate(0, 0)),
            // Test x toroidal translation without bounds
            (Coordinate(0, 0), Vector(-1, 0), Coordinate(9, 0)),
            (Coordinate(9, 0), Vector(1, 0), Coordinate(0, 0)),
            (Coordinate(0, 0), Vector(10, 0), Coordinate(0, 0)),
            (Coordinate(0, 0), Vector(-10, 0), Coordinate(0, 0)),
            // Test y toroidal translation within bounds
            (Coordinate(0, 0), Vector(0, 1), Coordinate(0, 1)),
            (Coordinate(0, 1), Vector(0, -1), Coordinate(0, 0)),
            // Test y toroidal translation without bounds
            (Coordinate(0, 0), Vector(0, -1), Coordinate(0, 9)),
            (Coordinate(0, 9), Vector(0, 1), Coordinate(0, 0)),
            (Coordinate(0, 0), Vector(0, 10), Coordinate(0, 0)),
            (Coordinate(0, 0), Vector(0, -10), Coordinate(0, 0)),
        ];

        data.iter().for_each(|(point, vector, result)| {
            assert_eq!(toroidal_translation(&point, &vector, &10, &10), *result)
        });
    }
}

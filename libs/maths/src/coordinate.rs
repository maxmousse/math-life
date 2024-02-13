#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate(pub usize, pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector(pub isize, pub isize);

pub fn vector(source: &Coordinate, destination: &Coordinate) -> Vector {
    let Coordinate(source_x, source_y) = source;
    let Coordinate(destination_x, destination_y) = destination;

    Vector(
        *destination_x as isize - *source_x as isize,
        *destination_y as isize - *source_y as isize,
    )
}

pub fn toroidal_translation(
    point: &Coordinate,
    vector: &Vector,
    plan_width: &usize,
    plan_height: &usize,
) -> Coordinate {
    let Coordinate(point_x, point_y) = point;
    let Vector(vector_x, vector_y) = vector;

    Coordinate(
        (*point_x as isize + vector_x).abs() as usize % plan_width,
        (*point_y as isize + vector_y).abs() as usize % plan_height,
    )
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
            (Coordinate(0, 0), Vector(11, 0), Coordinate(1, 0)),
            (Coordinate(1, 0), Vector(-11, 0), Coordinate(0, 0)),
            // Test y toroidal translation within bounds
            (Coordinate(0, 0), Vector(0, 1), Coordinate(0, 1)),
            (Coordinate(0, 1), Vector(0, -1), Coordinate(0, 0)),
            // Test y toroidal translation without bounds
            (Coordinate(0, 0), Vector(0, 11), Coordinate(0, 1)),
            (Coordinate(0, 1), Vector(0, -11), Coordinate(0, 0)),
        ];

        data.iter().for_each(|(point, vector, result)| {
            assert_eq!(toroidal_translation(&point, &vector, &10, &10), *result)
        });
    }
}

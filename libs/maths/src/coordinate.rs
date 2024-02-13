#[derive(Clone, Copy)]
pub struct Coordinate(pub usize, pub usize);

pub fn toroidal_translation(
    point: &Coordinate,
    vector: &Coordinate,
    plan_width: &usize,
    plan_height: &usize,
) -> Coordinate {
    let Coordinate(point_x, point_y) = point;
    let Coordinate(vector_x, vector_y) = vector;

    Coordinate(
        (point_x + vector_x) % plan_width,
        (point_y + vector_y) % plan_height,
    )
}

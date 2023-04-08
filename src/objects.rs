use crate::object;

object!(Cube {

});

impl Cube {
    pub fn new() -> Self {
        let mut cube = Cube::empty();
        cube.set_vertices(vec![
            -1.0, -1.0, -1.0,
            -1.0, -1.0, 1.0,
            -1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, 1.0,
            1.0, 1.0, -1.0,
            1.0, 1.0, 1.0,
        ].as_slice());
        cube.set_indices(vec![
            0, 1, 2,
            3, 2, 1,
            1, 5, 3,
            7, 3, 5,
            5, 4, 7,
            6, 7, 4,
            4, 0, 6,
            2, 6, 0,
            4, 5, 0,
            1, 0, 5,
            2, 3, 6,
            7, 6, 3,
        ].as_slice());
        cube
    }
}
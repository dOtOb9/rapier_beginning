use rapier2d::prelude::*;

fn main() {
    use nalgebra::{Isometry2, vector};

    let iso = Isometry2::new(vector![1.0, 2.0], 0.5);

    assert_eq!(iso.rotation.angle(), 0.5);
    assert_eq!(iso.translation.vector.x, 1.0);
    assert_eq!(iso.translation.vector.y, 2.0);
}
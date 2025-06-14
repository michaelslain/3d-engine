use crate::core::object::Object;
use crate::geometry::mesh::Mesh;

pub struct Pyramid;

impl Pyramid {
    pub fn new(base_size: f32, height: f32) -> Object {
        let half_base = base_size * 0.5;
        let mesh = Mesh::from_raw_coordinates(vec![
            // Base
            [
                -half_base, 0.0, -half_base, half_base, 0.0, -half_base, half_base, 0.0, half_base,
            ],
            [
                -half_base, 0.0, -half_base, half_base, 0.0, half_base, -half_base, 0.0, half_base,
            ],
            // Front face
            [
                0.0, height, 0.0, -half_base, 0.0, half_base, half_base, 0.0, half_base,
            ],
            // Back face
            [
                0.0, height, 0.0, half_base, 0.0, -half_base, -half_base, 0.0, -half_base,
            ],
            // Left face
            [
                0.0, height, 0.0, -half_base, 0.0, -half_base, -half_base, 0.0, half_base,
            ],
            // Right face
            [
                0.0, height, 0.0, half_base, 0.0, half_base, half_base, 0.0, -half_base,
            ],
        ]);
        Object::new(mesh)
    }
}

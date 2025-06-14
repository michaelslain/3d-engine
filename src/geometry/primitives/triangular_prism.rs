use crate::core::object::Object;
use crate::geometry::mesh::Mesh;

pub struct TriangularPrism;

impl TriangularPrism {
    pub fn new(base_width: f32, height: f32, depth: f32) -> Object {
        let half_width = base_width * 0.5;
        let half_depth = depth * 0.5;
        let mesh = Mesh::from_raw_coordinates(vec![
            // Front face
            [
                -half_width,
                0.0,
                half_depth,
                half_width,
                0.0,
                half_depth,
                0.0,
                height,
                half_depth,
            ],
            // Back face
            [
                -half_width,
                0.0,
                -half_depth,
                0.0,
                height,
                -half_depth,
                half_width,
                0.0,
                -half_depth,
            ],
            // Bottom face
            [
                -half_width,
                0.0,
                -half_depth,
                half_width,
                0.0,
                -half_depth,
                half_width,
                0.0,
                half_depth,
            ],
            [
                -half_width,
                0.0,
                -half_depth,
                half_width,
                0.0,
                half_depth,
                -half_width,
                0.0,
                half_depth,
            ],
            // Left face
            [
                -half_width,
                0.0,
                -half_depth,
                -half_width,
                0.0,
                half_depth,
                0.0,
                height,
                half_depth,
            ],
            [
                0.0,
                height,
                half_depth,
                0.0,
                height,
                -half_depth,
                -half_width,
                0.0,
                -half_depth,
            ],
            // Right face
            [
                half_width,
                0.0,
                -half_depth,
                0.0,
                height,
                -half_depth,
                0.0,
                height,
                half_depth,
            ],
            [
                0.0,
                height,
                half_depth,
                half_width,
                0.0,
                half_depth,
                half_width,
                0.0,
                -half_depth,
            ],
        ]);
        Object::new(mesh)
    }
}

use crate::core::object::Object;
use crate::geometry::mesh::Mesh;

pub struct Cube;

impl Cube {
    pub fn new(size: f32) -> Object {
        let half_size = size * 0.5;
        let mesh = Mesh::from_raw_coordinates(vec![
            // Front face (z = 0.5)
            [
                -half_size, -half_size, half_size, half_size, -half_size, half_size, half_size,
                half_size, half_size,
            ], // Triangle 1
            [
                -half_size, -half_size, half_size, half_size, half_size, half_size, -half_size,
                half_size, half_size,
            ], // Triangle 2
            // Back face (z = -0.5)
            [
                -half_size, -half_size, -half_size, -half_size, half_size, -half_size, half_size,
                half_size, -half_size,
            ], // Triangle 3
            [
                -half_size, -half_size, -half_size, half_size, half_size, -half_size, half_size,
                -half_size, -half_size,
            ], // Triangle 4
            // Left face (x = -0.5)
            [
                -half_size, -half_size, -half_size, -half_size, -half_size, half_size, -half_size,
                half_size, half_size,
            ], // Triangle 5
            [
                -half_size, -half_size, -half_size, -half_size, half_size, half_size, -half_size,
                half_size, -half_size,
            ], // Triangle 6
            // Right face (x = 0.5)
            [
                half_size, -half_size, -half_size, half_size, half_size, -half_size, half_size,
                half_size, half_size,
            ], // Triangle 7
            [
                half_size, -half_size, -half_size, half_size, half_size, half_size, half_size,
                -half_size, half_size,
            ], // Triangle 8
            // Top face (y = 0.5)
            [
                -half_size, half_size, -half_size, -half_size, half_size, half_size, half_size,
                half_size, half_size,
            ], // Triangle 9
            [
                -half_size, half_size, -half_size, half_size, half_size, half_size, half_size,
                half_size, -half_size,
            ], // Triangle 10
            // Bottom face (y = -0.5)
            [
                -half_size, -half_size, -half_size, half_size, -half_size, -half_size, half_size,
                -half_size, half_size,
            ], // Triangle 11
            [
                -half_size, -half_size, -half_size, half_size, -half_size, half_size, -half_size,
                -half_size, half_size,
            ], // Triangle 12
        ]);
        Object::new(mesh)
    }
}

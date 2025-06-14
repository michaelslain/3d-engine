use crate::core::object::Object;
use crate::geometry::mesh::Mesh;

pub struct RectangularPrism;

impl RectangularPrism {
    pub fn new(width: f32, height: f32, depth: f32) -> Object {
        let half_width = width * 0.5;
        let half_height = height * 0.5;
        let half_depth = depth * 0.5;

        let mesh = Mesh::from_raw_coordinates(vec![
            // Front face (z = half_depth)
            [
                -half_width,
                -half_height,
                half_depth,
                half_width,
                -half_height,
                half_depth,
                half_width,
                half_height,
                half_depth,
            ], // Triangle 1
            [
                -half_width,
                -half_height,
                half_depth,
                half_width,
                half_height,
                half_depth,
                -half_width,
                half_height,
                half_depth,
            ], // Triangle 2
            // Back face (z = -half_depth)
            [
                -half_width,
                -half_height,
                -half_depth,
                -half_width,
                half_height,
                -half_depth,
                half_width,
                half_height,
                -half_depth,
            ], // Triangle 3
            [
                -half_width,
                -half_height,
                -half_depth,
                half_width,
                half_height,
                -half_depth,
                half_width,
                -half_height,
                -half_depth,
            ], // Triangle 4
            // Left face (x = -half_width)
            [
                -half_width,
                -half_height,
                -half_depth,
                -half_width,
                -half_height,
                half_depth,
                -half_width,
                half_height,
                half_depth,
            ], // Triangle 5
            [
                -half_width,
                -half_height,
                -half_depth,
                -half_width,
                half_height,
                half_depth,
                -half_width,
                half_height,
                -half_depth,
            ], // Triangle 6
            // Right face (x = half_width)
            [
                half_width,
                -half_height,
                -half_depth,
                half_width,
                half_height,
                -half_depth,
                half_width,
                half_height,
                half_depth,
            ], // Triangle 7
            [
                half_width,
                -half_height,
                -half_depth,
                half_width,
                half_height,
                half_depth,
                half_width,
                -half_height,
                half_depth,
            ], // Triangle 8
            // Top face (y = half_height)
            [
                -half_width,
                half_height,
                -half_depth,
                -half_width,
                half_height,
                half_depth,
                half_width,
                half_height,
                half_depth,
            ], // Triangle 9
            [
                -half_width,
                half_height,
                -half_depth,
                half_width,
                half_height,
                half_depth,
                half_width,
                half_height,
                -half_depth,
            ], // Triangle 10
            // Bottom face (y = -half_height)
            [
                -half_width,
                -half_height,
                -half_depth,
                half_width,
                -half_height,
                -half_depth,
                half_width,
                -half_height,
                half_depth,
            ], // Triangle 11
            [
                -half_width,
                -half_height,
                -half_depth,
                half_width,
                -half_height,
                half_depth,
                -half_width,
                -half_height,
                half_depth,
            ], // Triangle 12
        ]);
        Object::new(mesh)
    }
}

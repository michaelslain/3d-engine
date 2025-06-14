use crate::core::object::Object;
use crate::geometry::mesh::Mesh;
use std::f32::consts::PI;

pub struct Cylinder;

impl Cylinder {
    pub fn new(radius: f32, height: f32, segments: u32) -> Object {
        let mut triangles = Vec::new();
        let angle_step = 2.0 * PI / segments as f32;

        // Generate vertices for top and bottom circles
        for i in 0..segments {
            let angle1 = i as f32 * angle_step;
            let angle2 = (i + 1) as f32 * angle_step;

            let x1 = radius * angle1.cos();
            let z1 = radius * angle1.sin();
            let x2 = radius * angle2.cos();
            let z2 = radius * angle2.sin();

            // Top face
            triangles.push([0.0, height, 0.0, x1, height, z1, x2, height, z2]);
            // Bottom face
            triangles.push([0.0, 0.0, 0.0, x2, 0.0, z2, x1, 0.0, z1]);
            // Side face
            triangles.push([x1, height, z1, x1, 0.0, z1, x2, 0.0, z2]);
            triangles.push([x1, height, z1, x2, 0.0, z2, x2, height, z2]);
        }

        let mesh = Mesh::from_raw_coordinates(triangles);
        Object::new(mesh)
    }
}

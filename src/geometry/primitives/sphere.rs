use crate::core::object::Object;
use crate::geometry::mesh::Mesh;
use std::f32::consts::PI;

pub struct Sphere;

impl Sphere {
    pub fn new(radius: f32, segments: u32) -> Object {
        let mut triangles = Vec::new();
        let angle_step = 2.0 * PI / segments as f32;
        let height_step = PI / segments as f32;

        for i in 0..segments {
            let phi1 = i as f32 * height_step;
            let phi2 = (i + 1) as f32 * height_step;

            for j in 0..segments {
                let theta1 = j as f32 * angle_step;
                let theta2 = (j + 1) as f32 * angle_step;

                // Generate vertices for this segment
                let v1 = [
                    radius * phi1.sin() * theta1.cos(),
                    radius * phi1.cos(),
                    radius * phi1.sin() * theta1.sin(),
                ];
                let v2 = [
                    radius * phi1.sin() * theta2.cos(),
                    radius * phi1.cos(),
                    radius * phi1.sin() * theta2.sin(),
                ];
                let v3 = [
                    radius * phi2.sin() * theta1.cos(),
                    radius * phi2.cos(),
                    radius * phi2.sin() * theta1.sin(),
                ];
                let v4 = [
                    radius * phi2.sin() * theta2.cos(),
                    radius * phi2.cos(),
                    radius * phi2.sin() * theta2.sin(),
                ];

                // Add triangles
                triangles.push([
                    v1[0], v1[1], v1[2], v2[0], v2[1], v2[2], v3[0], v3[1], v3[2],
                ]);
                triangles.push([
                    v2[0], v2[1], v2[2], v4[0], v4[1], v4[2], v3[0], v3[1], v3[2],
                ]);
            }
        }

        let mesh = Mesh::from_raw_coordinates(triangles);
        Object::new(mesh)
    }
}

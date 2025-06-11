use crate::core::config::{get_config, update_config};
use crate::core::object::Object;
use crate::geometry::{mesh::Mesh, triangle::Triangle};
use glam::{Mat4, Vec3, Vec4};
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Camera {
    near: f32, // distance from camera to near plane (1.0)
    far: f32,  // distance from camera to far plane (10.0)
}

impl Camera {
    pub fn new() -> Self {
        let camera = Self {
            near: 1.0, // Standard near plane at 1.0
            far: 10.0, // Standard far plane at 10.0
        };

        camera
    }

    pub fn project_point(&self, point: Vec3) -> Vec3 {
        let config = get_config();

        let a = config.width as f32 / config.height as f32;
        let f = 1.0 / (config.fov * 0.5 * PI / 180.0).tan();
        let q = self.far / (self.far - self.near);

        let matrix = Mat4::from_cols(
            Vec4::new(f / a, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, q, 1.0),
            Vec4::new(0.0, 0.0, -self.near * q, 0.0),
        );

        let clip = matrix * point.extend(1.0);
        let projected_point = clip.truncate() / clip.w; // perspective divide

        projected_point
    }

    pub fn project_triangle(&self, triangle: Triangle) -> Triangle {
        Triangle {
            vertices: [
                self.project_point(triangle.vertices[0]),
                self.project_point(triangle.vertices[1]),
                self.project_point(triangle.vertices[2]),
            ],
        }
    }

    pub fn project_mesh(&self, mesh: Mesh) -> Mesh {
        Mesh {
            triangles: mesh
                .triangles
                .into_iter()
                .map(|triangle| self.project_triangle(triangle))
                .collect(),
        }
    }

    pub fn project_object(&self, object: &Object) -> Mesh {
        let mut projected_mesh = Mesh::from_raw_coordinates(Vec::new());
        for triangle in object.get_mesh().triangles.iter() {
            let v0 = self.project_point(triangle.vertices[0] + object.get_position());
            let v1 = self.project_point(triangle.vertices[1] + object.get_position());
            let v2 = self.project_point(triangle.vertices[2] + object.get_position());
            projected_mesh.triangles.push(Triangle {
                vertices: [v0, v1, v2],
            });
        }
        projected_mesh
    }
}

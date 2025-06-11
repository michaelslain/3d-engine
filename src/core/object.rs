use crate::geometry::mesh::Mesh;
use glam::{Mat3, Vec3};

#[derive(Clone)]
pub struct Object {
    mesh: Mesh,
    position: Vec3,
    rotation: Vec3,
}

impl Object {
    pub fn new(mesh: Mesh) -> Self {
        Object {
            mesh,
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
        }
    }

    pub fn update(&mut self, delta_time: f32) {}

    pub fn get_mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn get_position(&self) -> &Vec3 {
        &self.position
    }

    pub fn get_rotation(&self) -> &Vec3 {
        &self.rotation
    }

    pub fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = mesh;
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
    }

    pub fn transformed_triangles(&self) -> Vec<crate::geometry::triangle::Triangle> {
        let rotate_matrix_x = Mat3::from_rotation_x(self.rotation.x);
        let rotate_matrix_y = Mat3::from_rotation_y(self.rotation.y);
        let rotate_matrix_z = Mat3::from_rotation_z(self.rotation.z);

        // Combine rotation matrices
        let rotation = rotate_matrix_z * rotate_matrix_y * rotate_matrix_x;

        self.mesh
            .triangles
            .iter()
            .map(|tri| crate::geometry::triangle::Triangle {
                vertices: [
                    rotation * tri.vertices[0] + self.position,
                    rotation * tri.vertices[1] + self.position,
                    rotation * tri.vertices[2] + self.position,
                ],
            })
            .collect()
    }
}

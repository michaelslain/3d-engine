use crate::geometry::mesh::Mesh;
use crate::geometry::triangle::Triangle;
use glam::{Mat3, Vec3};

pub struct Object {
    mesh: Mesh,
    position: Vec3,
    rotation: Vec3,
    update: Option<Box<dyn FnMut(&mut Self, f32)>>,
}

impl Object {
    pub fn new(mesh: Mesh) -> Self {
        Object {
            mesh,
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            update: None,
        }
    }

    pub fn set_update<F>(&mut self, f: F)
    where
        F: FnMut(&mut Self, f32) + 'static,
    {
        self.update = Some(Box::new(f));
    }

    pub fn update(&mut self, delta_time: f32) {
        if let Some(mut f) = self.update.take() {
            f(self, delta_time);
            self.update = Some(f);
        }
    }

    pub fn transformed_triangle(&self, triangle: Triangle) -> Triangle {
        let vertices = triangle.get_vertices();
        let rotate_matrix_x = Mat3::from_rotation_x(self.rotation.x);
        let rotate_matrix_y = Mat3::from_rotation_y(self.rotation.y);
        let rotate_matrix_z = Mat3::from_rotation_z(self.rotation.z);

        let rotation: Mat3 = rotate_matrix_z * rotate_matrix_y * rotate_matrix_x;

        crate::geometry::triangle::Triangle::new([
            rotation * vertices[0] + self.position,
            rotation * vertices[1] + self.position,
            rotation * vertices[2] + self.position,
        ])
    }

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
}

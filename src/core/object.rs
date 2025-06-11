use crate::geometry::mesh::Mesh;
use glam::Vec3;

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

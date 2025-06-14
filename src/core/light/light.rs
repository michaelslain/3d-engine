use crate::core::object::Object;
use crate::geometry::mesh::Mesh;
use glam::Vec3;

pub trait Light {
    fn get_color(&self) -> Vec3;
    fn get_intensity(&self) -> f32;
    fn get_position(&self) -> &Vec3;
    fn get_rotation(&self) -> &Vec3;
    fn get_direction(&self) -> Vec3;
}

pub struct BaseLight {
    object: Object,
    color: Vec3,
    intensity: f32,
}

impl BaseLight {
    pub fn new() -> Self {
        Self {
            object: Object::new(Mesh::new(Vec::new())),
            color: Vec3::new(1.0, 1.0, 1.0), // White light by default
            intensity: 1.0,
        }
    }

    pub fn set_color(&mut self, color: Vec3) {
        self.color = color;
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    pub fn get_color(&self) -> Vec3 {
        self.color
    }

    pub fn get_intensity(&self) -> f32 {
        self.intensity
    }

    // Delegate position and rotation methods to the inner object
    pub fn set_position(&mut self, position: Vec3) {
        self.object.set_position(position);
    }

    pub fn get_position(&self) -> &Vec3 {
        self.object.get_position()
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.object.set_rotation(rotation);
    }

    pub fn get_rotation(&self) -> &Vec3 {
        self.object.get_rotation()
    }
}

impl Light for BaseLight {
    fn get_color(&self) -> Vec3 {
        self.color
    }

    fn get_intensity(&self) -> f32 {
        self.intensity
    }

    fn get_position(&self) -> &Vec3 {
        self.object.get_position()
    }

    fn get_rotation(&self) -> &Vec3 {
        self.object.get_rotation()
    }

    fn get_direction(&self) -> Vec3 {
        Vec3::new(0.0, -1.0, 0.0) // Default downward direction
    }
}

use crate::core::light::light::{BaseLight, Light};
use glam::Vec3;

pub struct DirectionalLight {
    base: BaseLight,
    direction: Vec3,
}

impl DirectionalLight {
    pub fn new(direction: Vec3) -> Self {
        let normalized_direction = direction.normalize();

        Self {
            base: BaseLight::new(),
            direction: normalized_direction,
        }
    }

    pub fn set_color(&mut self, color: Vec3) {
        self.base.set_color(color);
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.base.set_intensity(intensity);
    }

    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction.normalize(); // Ensure direction is normalized
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.base.set_position(position);
    }

    pub fn get_position(&self) -> &Vec3 {
        self.base.get_position()
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.base.set_rotation(rotation);
    }

    pub fn get_rotation(&self) -> &Vec3 {
        self.base.get_rotation()
    }
}

impl Light for DirectionalLight {
    fn get_color(&self) -> Vec3 {
        self.base.get_color()
    }

    fn get_intensity(&self) -> f32 {
        self.base.get_intensity()
    }

    fn get_position(&self) -> &Vec3 {
        self.base.get_position()
    }

    fn get_rotation(&self) -> &Vec3 {
        self.base.get_rotation()
    }

    fn get_direction(&self) -> Vec3 {
        self.direction
    }
}

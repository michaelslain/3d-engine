mod core;
mod engine;
mod geometry;

use crate::core::camera::Camera;
use crate::core::light::DirectionalLight;
use crate::core::scene::Scene;
use crate::engine::Engine;
use crate::geometry::primitives::cube::Cube;
use glam::Vec3;

fn main() {
    let mut scene = Scene::new();

    // Add a directional light
    let mut light = DirectionalLight::new(Vec3::new(1.0, -1.0, 1.0));
    light.set_color(Vec3::new(1.0, 1.0, 1.0));
    light.set_intensity(1.0);
    scene.add_light(light);

    // Create a rotating cube
    let mut cube = Cube::new(1.0);
    cube.set_position(Vec3::new(0.0, 0.0, 5.0));
    cube.set_update(|obj, delta_time| {
        let current_rotation = *obj.get_rotation();
        obj.set_rotation(Vec3::new(
            current_rotation.x + delta_time,
            current_rotation.y + delta_time * 0.5,
            current_rotation.z,
        ));
    });
    scene.add_object(cube);

    let camera = Camera::new();
    let engine = Engine::new(scene, camera);
    engine.run();
}

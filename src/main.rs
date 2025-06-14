mod core;
mod engine;
mod geometry;
mod scene;

use crate::core::camera::Camera;
use crate::engine::Engine;
use crate::geometry::primitives::cube::Cube;
use crate::scene::Scene;
use glam::Vec3;

fn main() {
    let mut scene = Scene::new();

    let mut cube1 = Cube::new(1.0);
    cube1.set_position(Vec3::new(1.0, 0.0, 3.0));
    cube1.set_update(|obj, dt| {
        let rot = obj.get_rotation();
        obj.set_rotation(*rot + Vec3::new(dt, dt, dt));
    });

    let mut cube2 = Cube::new(1.0);
    cube2.set_position(Vec3::new(-3.0, 0.0, 4.0));
    cube2.set_update(|obj, dt| {
        let rot = obj.get_rotation();
        obj.set_rotation(*rot + Vec3::new(dt, dt, dt));
    });

    scene.add_object(cube1);
    scene.add_object(cube2);

    let camera = Camera::new();
    let engine = Engine::new(scene, camera);
    engine.run();
}

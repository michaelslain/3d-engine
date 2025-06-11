mod core;
mod engine;
mod geometry;
mod scene;

use crate::core::camera::Camera;
use crate::core::object::Object;
use crate::engine::Engine;
use crate::geometry::mesh::Mesh;
use crate::scene::Scene;
use glam::Vec3;

fn main() {
    let mut scene = Scene::new();

    let mut cube1: Object = Object::new(Mesh::from_raw_coordinates(vec![
        // Front face (z = 0.5)
        [-0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5], // Triangle 1
        [-0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5], // Triangle 2
        // Back face (z = -0.5)
        [-0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5], // Triangle 3
        [-0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5], // Triangle 4
        // Left face (x = -0.5)
        [-0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5], // Triangle 5
        [-0.5, -0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5], // Triangle 6
        // Right face (x = 0.5)
        [0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5], // Triangle 7
        [0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5], // Triangle 8
        // Top face (y = 0.5)
        [-0.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5], // Triangle 9
        [-0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5], // Triangle 10
        // Bottom face (y = -0.5)
        [-0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5], // Triangle 11
        [-0.5, -0.5, -0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5], // Triangle 12
    ]));

    cube1.set_position(Vec3::new(1.0, 0.0, 3.0));
    cube1.set_rotation(Vec3::new(10.0, 1.0, 2.0));

    cube1.set_update(|obj, dt| {
        let rot = obj.get_rotation();
        obj.set_rotation(*rot + Vec3::new(0.0, dt, 0.0));
    });

    let mut cube2: Object = Object::new(Mesh::from_raw_coordinates(vec![
        // Front face (z = 0.5)
        [-0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5], // Triangle 1
        [-0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5], // Triangle 2
        // Back face (z = -0.5)
        [-0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5], // Triangle 3
        [-0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5], // Triangle 4
        // Left face (x = -0.5)
        [-0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5], // Triangle 5
        [-0.5, -0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5], // Triangle 6
        // Right face (x = 0.5)
        [0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5], // Triangle 7
        [0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5], // Triangle 8
        // Top face (y = 0.5)
        [-0.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5], // Triangle 9
        [-0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5], // Triangle 10
        // Bottom face (y = -0.5)
        [-0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5], // Triangle 11
        [-0.5, -0.5, -0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5], // Triangle 12
    ]));

    cube2.set_position(Vec3::new(-1.0, 0.0, 4.0));
    cube2.set_rotation(Vec3::new(0.0, 0.0, 0.0));

    cube2.set_update(|obj, dt| {
        let rot = obj.get_rotation();
        obj.set_rotation(*rot + Vec3::new(dt, 0.0, 0.0));
    });

    scene.add_object(cube1);
    scene.add_object(cube2);

    let camera = Camera::new();
    let engine = Engine::new(scene, camera);
    engine.run();
}

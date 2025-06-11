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

    let mut cube = Object::new(Mesh::from_raw_coordinates(vec![
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

    cube.set_position(Vec3::new(1.0, 0.0, 3.0));
    cube.set_rotation(Vec3::new(0.0, 0.0, 2.0));

    scene.add_object(cube);

    let camera = Camera::new();
    let engine = Engine::new(scene, camera);
    engine.run();
}

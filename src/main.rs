mod core;
mod engine;
mod geometry;
mod scene;

use crate::core::camera::Camera;
use crate::engine::Engine;
use crate::geometry::primitives::{
    cube::Cube, cylinder::Cylinder, pyramid::Pyramid, rectangular_prism::RectangularPrism,
    sphere::Sphere, triangular_prism::TriangularPrism,
};
use crate::scene::Scene;
use glam::Vec3;

fn main() {
    let mut scene = Scene::new();

    // Cube
    let mut cube = Cube::new(1.0);
    cube.set_position(Vec3::new(5.0, 0.0, 10.0));
    scene.add_object(cube);

    // Rectangular Prism
    let mut rect_prism = RectangularPrism::new(1.0, 2.0, 1.0);
    rect_prism.set_position(Vec3::new(8.0, 0.0, 8.0));
    scene.add_object(rect_prism);

    // Pyramid
    let mut pyramid = Pyramid::new(1.0, 2.0);
    pyramid.set_position(Vec3::new(10.0, 0.0, 5.0));
    scene.add_object(pyramid);

    // Triangular Prism
    let mut tri_prism = TriangularPrism::new(1.0, 2.0, 1.0);
    tri_prism.set_position(Vec3::new(0.0, 0.0, 2.0));
    scene.add_object(tri_prism);

    // Cylinder
    let mut cylinder = Cylinder::new(0.5, 2.0, 16);
    cylinder.set_position(Vec3::new(-5.0, 0.0, 5.0));
    scene.add_object(cylinder);

    // Sphere
    let mut sphere = Sphere::new(0.5, 16);
    sphere.set_position(Vec3::new(2.0, 0.0, 5.0));
    scene.add_object(sphere);

    let camera = Camera::new();
    let engine = Engine::new(scene, camera);
    engine.run();
}

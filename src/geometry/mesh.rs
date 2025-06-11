use crate::geometry::triangle::Triangle;
use glam::Vec3;

#[derive(Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        Mesh { triangles }
    }

    pub fn from_raw_coordinates(triangles: Vec<[f32; 9]>) -> Self {
        let triangles = triangles
            .into_iter()
            .map(|coords| Triangle {
                vertices: [
                    Vec3::new(coords[0], coords[1], coords[2]),
                    Vec3::new(coords[3], coords[4], coords[5]),
                    Vec3::new(coords[6], coords[7], coords[8]),
                ],
            })
            .collect();
        Mesh { triangles }
    }

    pub fn get_vertices(&self) -> Vec<Vec3> {
        let mut vertices = Vec::new();
        for triangle in &self.triangles {
            // Edge 1: v0 -> v1
            vertices.push(triangle.vertices[0]);
            vertices.push(triangle.vertices[1]);
            // Edge 2: v1 -> v2
            vertices.push(triangle.vertices[1]);
            vertices.push(triangle.vertices[2]);
            // Edge 3: v2 -> v0
            vertices.push(triangle.vertices[2]);
            vertices.push(triangle.vertices[0]);
        }
        vertices
    }
}

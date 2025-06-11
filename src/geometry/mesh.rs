use crate::geometry::triangle::Triangle;
use glam::Vec3;

#[derive(Clone)]
pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        Mesh { triangles }
    }

    pub fn from_raw_coordinates(triangles: Vec<[f32; 9]>) -> Self {
        let triangles = triangles
            .into_iter()
            .map(|coords| {
                Triangle::new([
                    Vec3::new(coords[0], coords[1], coords[2]),
                    Vec3::new(coords[3], coords[4], coords[5]),
                    Vec3::new(coords[6], coords[7], coords[8]),
                ])
            })
            .collect();
        Mesh { triangles }
    }

    pub fn get_vertices(&self) -> Vec<Vec3> {
        let mut vertices = Vec::new();
        for triangle in &self.triangles {
            let [v0, v1, v2] = triangle.get_vertices();
            // Add edges of the triangle
            vertices.push(v0);
            vertices.push(v1);
            vertices.push(v1);
            vertices.push(v2);
            vertices.push(v2);
            vertices.push(v0);
        }
        vertices
    }

    pub fn get_visible_vertices(&self) -> Vec<Vec3> {
        let mut vertices = Vec::new();
        for triangle in &self.triangles {
            if triangle.get_normal().z < 0.0 {
                let [v0, v1, v2] = triangle.get_vertices();
                vertices.push(v0);
                vertices.push(v1);
                vertices.push(v1);
                vertices.push(v2);
                vertices.push(v2);
                vertices.push(v0);
            }
        }
        vertices
    }

    pub fn get_triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    pub fn set_triangles(&mut self, triangles: Vec<Triangle>) {
        self.triangles = triangles;
    }
}

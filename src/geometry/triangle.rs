use glam::Vec3;

#[derive(Clone)]
pub struct Triangle {
    vertices: [Vec3; 3],
    normal: Vec3,
}

impl Triangle {
    pub fn new(vertices: [Vec3; 3]) -> Self {
        let mut triangle = Self {
            vertices,
            normal: Vec3::ZERO,
        };
        triangle.calculate_normal();
        triangle
    }

    fn calculate_normal(&mut self) {
        let a = self.vertices[1] - self.vertices[0];
        let b = self.vertices[2] - self.vertices[0];

        let normal = a.cross(b);

        self.normal = normal.normalize();
    }

    pub fn get_vertices(&self) -> [Vec3; 3] {
        self.vertices
    }

    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    pub fn set_vertices(&mut self, vertices: [Vec3; 3]) {
        self.vertices = vertices;
        self.calculate_normal();
    }
}
